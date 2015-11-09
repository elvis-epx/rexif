use super::types::*;
use super::lowlevel::*;
use super::debug::*;
use super::tiff::parse_exif_ifd;
use super::tiff::parse_ifd;


/// Parse the fake TIFF's IFD0 and looks for Nikon Sub IFDs
pub fn parse_nikon_ifd(le: bool, ifd0_offset: usize, contents: &[u8],
			exif_entries: &mut Vec<ExifEntry>)
{
	let mut offset = ifd0_offset;

	if contents.len() < offset + 2 {
		warning("Nikon: no IFD0 count in tiff");
		return;
	}

	let count = read_u16(le, &contents[offset..offset + 2]);
	let ifd_length = (count as usize) * 12 + 4;
	offset += 2;

	if contents.len() < (offset + ifd_length) {
		warning("Nikon: IFD0: buffer too short for IFD0 count!");
		return;
	}

	// At this point we don't know the Nikon Format yet, so passing
	// Namespace::NikonFormat2 is just to satisfy the API. Nikon Format 2
	// is also the default if no version tag is found.

	let mut ns = Namespace::NikonFormat2;

	let (mut ifd, _) = parse_ifd(ns, false, le, count,
				&contents[offset..offset + ifd_length]);

	for entry in &mut ifd {
		if ! entry.copy_data(&contents) {
			warning(&format!("Could not copy data for {:x}", entry.tag));
			continue;
		}
		if entry.tag == 0x0001 &&
				entry.format == IfdFormat::Undefined &&
				entry.data.len() == 4 && 
				entry.data[0] == 0x30u8 &&
				entry.data[1] == 0x32u8 &&
				entry.data[2] == 0x31u8 &&
				entry.data[3] == 0x31u8 {
			ns = Namespace::NikonFormat3;
			warning("Nikon version 3");
		}
	}

	// Rescan IFD0 with right namespace/version

	// Get data tags in IFD0
	let _ = parse_exif_ifd(ns, le, contents, ifd0_offset, exif_entries);

	// Find subfields
	for entry in &ifd {
		warning(&format!("Nikon root tag 0x{:x} len {}", entry.tag, entry.data.len()));

		if entry.tag == ((ExifTag::NikonVr) as u32 & 0xffff) as u16 {
			warning(&format!("Parsing Nikon VR subfields"));
			// TODO parse subfields (compound format within Undefined; not IFD)
		}
		// TODO add other subfields
		// TODO synthetize an IFD in order to parse_exif_ifd to process it
	}
}

/// Parse the fake TIFF that is part of a Nikon Makernote tag blob
fn parse_nikon_tiff(contents: &[u8], exif_entries: &mut Vec<ExifEntry>) -> bool
{
	// contents have at least 8 bytes at this point

	let mut le = false;

	if contents.len() < 8 {
		warning("Nikon: too short for a tiff");
		return false;
	} else if contents[0] == ('I' as u8) &&
			contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		le = true;
	} else if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
	} else {
		warning("Nikon makernote: preamble not tiff");
		return false;
	}

	let offset = read_u32(le, &contents[4..8]) as usize;

	let _ = parse_nikon_ifd(le, offset, &contents, exif_entries);

	return true;
}

fn hex(numbers: &[u8]) -> String
{
	let mut s = "".to_string();
	let mut first = true;
	for number in numbers {
		if !first {
			s = s + ", ";
		}
		first = false;
		let s2 = format!("{:02x}", number);
		s = s + &s2;
	}

	return s;
}

/// Parses a Nikon MakerNote tag.
pub fn nikon_makernote(raw: &Vec<u8>, main_le: bool, exif_entries: &mut Vec<ExifEntry>)
{
	// assuming newer format (embedded TIFF)
	warning("Nikon");

	// raw has at least 18 bytes at this point, so TIFF has at least 8 bytes

	if ! parse_nikon_tiff(&raw[10..], exif_entries) {

		// FIXME to enable older Nikon format, the top-level TIFF buffer
		// must be passed, because offsets are relative to the main TIFF,
		// not to MakerNote contents.

		// warning("Nikon: makernote not tiff, trying IFD@8 variant");
		// let _ = parse_nikon_ifd(main_le, 8, &raw[..], exif_entries);
	}
}

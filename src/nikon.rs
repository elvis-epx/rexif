use super::types::*;
use super::lowlevel::*;
use super::debug::*;
use super::tiff::parse_exif_ifd;

/// Parse the fake TIFF that is part of a Nikon Makernote tag blob
fn parse_nikon_tiff(contents: &[u8], exif_entries: &mut Vec<ExifEntry>)
{
	let mut le = false;

	if contents.len() < 8 {
		warning("Nikon makernote can't contain a tiff");
		return;
	} else if contents[0] == ('I' as u8) &&
			contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		le = true;
	} else if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
	} else {
		warning("Nikon makernote: bad preamble");
		return;
	}

	let offset = read_u32(le, &contents[4..8]) as usize;

	// injects the IFD back into normal parsing; error not propagated
	let _ = parse_exif_ifd(Namespace::Nikon, le, &contents, offset, exif_entries);
}

/// Parses a Nikon MakerNote tag.
pub fn nikon_makernote(raw: &Vec<u8>, _: bool, exif_entries: &mut Vec<ExifEntry>)
{
	// assuming newer format (embedded TIFF)
	parse_nikon_tiff(&raw[8..], exif_entries);
}

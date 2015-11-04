use super::types::*;
use super::types_impl::*;
use super::lowlevel::*;
use super::ifdformat::*;
use super::debug::*;
use super::exif::*;
use super::exifpost::*;

type InExifResult = Result<(), ExifError>;

/// Parse of raw IFD entry into EXIF data, if it is of a known type, and returns
/// an ExifEntry object. If the tag is unknown, the enumeration is set to UnknownToMe,
/// but the raw information of tag is still available in the ifd member.
pub fn parse_exif_entry(f: &IfdEntry) -> ExifEntry
{
	let (value, readable_value) = tag_value_new(f);

	let mut e = ExifEntry {
			ifd: f.clone(),
			tag: ExifTag::UnknownToMe,
			value: value,
			unit: "Unknown".to_string(),
			tag_readable: format!("Unparsed tag {:x}", f.tag).to_string(),
			value_readable: readable_value.clone(),
			value_more_readable: readable_value.clone(),
			};

	let (tag, tag_readable, unit, format, min_count, max_count, more_readable) = tag_to_exif(f.tag);

	if tag == ExifTag::UnknownToMe {
		// Unknown EXIF tag type
		return e;
	}

	// Internal assert:
	// 1) tag must match enum
	// 2) all types except Ascii, Undefined, Unknown must have definite length
	// 3) Str type must not have a definite length
	if (tag as u16) != f.tag ||
		(min_count == -1 && (format != IfdFormat::Ascii &&
				format != IfdFormat::Undefined &&
				format != IfdFormat::Unknown)) ||
		(min_count != -1 && format == IfdFormat::Ascii) {
		panic!("Internal error {:x}", f.tag);
	}

	if format != f.format {
		warning(&format!("EXIF tag {:x} {}, expected format {}, found {}",
			f.tag, tag_readable, format as u8, f.format as u8));
		return e;
	}

	if min_count != -1 &&
			((f.count as i32) < min_count ||
			(f.count as i32) > max_count) {
		warning(&format!("EXIF tag {:x} {}, format {}, expected count {}..{} found {}",
			f.tag, tag_readable, format as u8, min_count,
			max_count, f.count));
		return e;
	}

	e.tag = tag;
	e.tag_readable = tag_readable.to_string();
	e.unit = unit.to_string();
	e.value_more_readable = more_readable(&e.value, &readable_value);

	return e;
}

/// Superficial parse of IFD that can't fail
pub fn parse_ifd(subifd: bool, le: bool, count: u16, contents: &[u8]) -> (Vec<IfdEntry>, usize)
{
	let mut entries: Vec<IfdEntry> = Vec::new();

	for i in 0..count {
		// println!("Parsing IFD entry {}", i);
		let mut offset = (i as usize) * 12;
		let tag = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let format = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let count = read_u32(le, &contents[offset..offset + 4]);
		offset += 4;
		let data = &contents[offset..offset + 4];
		let data = data.to_vec();

		let entry = IfdEntry{tag: tag, format: ifdformat_new(format), count: count,
					ifd_data: data, le: le,
					ext_data: Vec::new(), data: Vec::new()};
		entries.push(entry);
	}

	let next_ifd = match subifd {
		true => 0,
		false => read_u32(le, &contents[count as usize * 12..]) as usize
	};

	return (entries, next_ifd);
}

/// Deep parse of IFD that grabs EXIF data from IFD0, SubIFD and GPS IFD
fn parse_exif_ifd(le: bool, contents: &[u8], ioffset: usize,
				exif_entries: &mut Vec<ExifEntry>) -> InExifResult
{
	let mut offset = ioffset;

	// println!("Offset is {}", offset);
	if contents.len() < (offset + 2) {
		return Err(ExifError{
			kind: ExifErrorKind::ExifIfdTruncated,
			extra: "Truncated at dir entry count".to_string()});
	}

	let count = read_u16(le, &contents[offset..offset + 2]);
	// println!("IFD entry count is {}", count);
	let ifd_length = (count as usize) * 12;
	offset += 2;

	if contents.len() < (offset + ifd_length) {
		return Err(ExifError{
			kind: ExifErrorKind::ExifIfdTruncated,
			extra: "Truncated at dir listing".to_string()});
	}

	let (mut ifd, _) = parse_ifd(true, le, count, &contents[offset..offset + ifd_length]);

	for entry in &mut ifd {
		entry.copy_data(&contents);
		let exif_entry = parse_exif_entry(&entry);
		exif_entries.push(exif_entry);
	}

	return Ok(());
}

/// Parses IFD0 and looks for SubIFD or GPS IFD within IFD0
pub fn parse_ifds(le: bool, ifd0_offset: usize, contents: &[u8]) -> ExifEntryResult
{
	let mut offset = ifd0_offset;
	let mut exif_entries: Vec<ExifEntry> = Vec::new();

	// fills exif_entries with data from IFD0

	match parse_exif_ifd(le, &contents, offset, &mut exif_entries) {
		Ok(_) => true,
		Err(e) => return Err(e),
	};

	// at this point we knot that IFD0 is good
	// looks for SubIFD (EXIF)

	let count = read_u16(le, &contents[offset..offset + 2]);
	let ifd_length = (count as usize) * 12 + 4;
	offset += 2;

	let (ifd, _) = parse_ifd(false, le, count, &contents[offset..offset + ifd_length]);

	for entry in &ifd {
		if entry.tag != (ExifTag::ExifOffset as u16) &&
				entry.tag != (ExifTag::GPSOffset as u16) {
			continue;
		}

		let exif_offset = entry.data_as_offset();

		if contents.len() < exif_offset {
			return Err(ExifError{
				kind: ExifErrorKind::ExifIfdTruncated,
				extra: "Exif SubIFD goes past EOF".to_string()});
		}

		match parse_exif_ifd(le, &contents, exif_offset, &mut exif_entries) {
			Ok(_) => true,
			Err(e) => return Err(e),
		};
	}

	// I didn't want to make the copy, but how to pass a vector that is
	// being iterated onto?
	let exif_entries_copy = exif_entries.clone();

	for entry in &mut exif_entries {
		exif_postprocessing(entry, &exif_entries_copy);
	}

	return Ok(exif_entries);
}

/// Parse a TIFF image, or embedded TIFF in JPEG, in order to get IFDs and then the EXIF data
pub fn parse_tiff(contents: &[u8]) -> ExifEntryResult
{
	let mut le = false;

	if contents.len() < 8 {
		return Err(ExifError{
			kind: ExifErrorKind::TiffTruncated,
			extra: "".to_string()});
	} else if contents[0] == ('I' as u8) &&
			contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		le = true;
	} else if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
	} else {
		let err = format!("Preamble is {:x} {:x} {:x} {:x}",
			contents[0], contents[1],
			contents[2], contents[3]);
		return Err(ExifError{
			kind: ExifErrorKind::TiffBadPreamble,
			extra: err.to_string()});
	}

	let offset = read_u32(le, &contents[4..8]) as usize;

	return parse_ifds(le, offset, &contents);
}

use super::types::*;
use super::lowlevel::*;
use super::exifpost::*;

type InExifResult = Result<(), ExifError>;

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

		let entry = IfdEntry{namespace: Namespace::Standard,
					tag: tag, format: IfdFormat::new(format),
					count: count, le: le, data: data};
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
		return Err(ExifError::ExifIfdTruncated("Truncated at dir entry count".to_string()))
	}

	let count = read_u16(le, &contents[offset..offset + 2]);
	// println!("IFD entry count is {}", count);
	let ifd_length = (count as usize) * 12;
	offset += 2;

	if contents.len() < (offset + ifd_length) {
		return Err(ExifError::ExifIfdTruncated("Truncated at dir listing".to_string()));
	}

	let (ifd, _) = parse_ifd(true, le, count, &contents[offset..offset + ifd_length]);

	for mut entry in ifd {
		if !entry.copy_data(&contents) {
			// data is probably beyond EOF
			continue;
		}
		let exif_entry = entry.into_exif_entry();
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
		if entry.tag != (((ExifTag::ExifOffset as u32) & 0xffff) as u16) &&
				entry.tag != (((ExifTag::GPSInfo as u32) & 0xffff) as u16) {
			continue;
		}

		let exif_offset = entry.data_as_offset();

		if contents.len() < exif_offset {
			return Err(ExifError::ExifIfdTruncated("Exif SubIFD goes past EOF".to_string()));
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
		return Err(ExifError::TiffTruncated);
	} else if contents[0] == b'I' &&
			contents[1] == b'I' &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		le = true;
	} else if contents[0] == b'M' && contents[1] == b'M' &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
	} else {
		let err = format!("Preamble is {:x} {:x} {:x} {:x}",
			contents[0], contents[1],
			contents[2], contents[3]);
		return Err(ExifError::TiffBadPreamble(err.to_string()));
	}

	let offset = read_u32(le, &contents[4..8]) as usize;

	return parse_ifds(le, offset, &contents);
}

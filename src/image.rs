use types::ExifError;

/// Detect the type of an image contained in a byte buffer
pub fn detect_type(contents: &Vec<u8>) -> &str
{
	if contents.len() < 11 {
		return "";
	}

	if contents[0] == 0xff && contents[1] == 0xd8 &&
			contents[2] == 0xff && // contents[3] == 0xe0 &&
			contents[6] == ('J' as u8) && contents[7] == ('F' as u8) &&
			contents[8] == ('I' as u8) && contents[9] == ('F' as u8) &&
			contents[10] == 0 {
		return "image/jpeg";
	}
	if contents[0] == 0xff && contents[1] == 0xd8 &&
			contents[2] == 0xff && // contents[3] == 0xe0 &&
			contents[6] == ('E' as u8) && contents[7] == ('x' as u8) &&
			contents[8] == ('i' as u8) && contents[9] == ('f' as u8) &&
			contents[10] == 0 {
		return "image/jpeg";
	}
	if contents[0] == ('I' as u8) && contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		return "image/tiff";
	}
	if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
		return "image/tiff";
	}

	return "";
}

/// Find the embedded TIFF in a JPEG image (that in turn contains the EXIF data)
pub fn find_embedded_tiff_in_jpeg(contents: &Vec<u8>)
                                  -> Result<(usize, usize), ExifError>
{
	let mut offset = 2 as usize;

	while offset < contents.len() {
		if contents.len() < (offset + 4) {
			return Err(ExifError::JpegWithoutExif("JPEG truncated in marker header".to_string()))
		}

		let marker: u16 = (contents[offset] as u16) * 256 + (contents[offset + 1] as u16);

		if marker < 0xff00 {
			return Err(ExifError::JpegWithoutExif(format!("Invalid marker {:x}", marker)))
		}

		offset += 2;
		let mut size = (contents[offset] as usize) * 256 + (contents[offset + 1] as usize);

		if size < 2 {
			return Err(ExifError::JpegWithoutExif("JPEG marker size must be at least 2 (because of the size word)".to_string()))
		}
		if contents.len() < (offset + size) {
			return Err(ExifError::JpegWithoutExif("JPEG truncated in marker body".to_string()))
		}

		if marker == 0xffe1 {
			// Discard the size word
			offset += 2;
			size -= 2;

			if size < 6 {
				return Err(ExifError::JpegWithoutExif("EXIF preamble truncated".to_string()))
			}

			if contents[offset + 0] != ('E' as u8) &&
					contents[offset + 1] != ('x' as u8) &&
					contents[offset + 2] != ('i' as u8) &&
					contents[offset + 3] != ('f' as u8) &&
					contents[offset + 4] != 0 &&
					contents[offset + 5] != 0 {
				return Err(ExifError::JpegWithoutExif("EXIF preamble unrecognized".to_string()))
			}

			// Discard the 'Exif\0\0' preamble
			offset += 6;
			size -= 6;

			return Ok((offset, size));
		}
		if marker == 0xffda {
			// last marker
			return Err(ExifError::JpegWithoutExif("Last mark found and no EXIF".to_string()))
		}
		offset += size;
	}

	return Err(ExifError::JpegWithoutExif("Scan past EOF and no EXIF found".to_string()))
}

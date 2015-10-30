use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
use std::result::Result;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::cell::RefCell;

pub struct ExifData {
	pub file: String,
	pub size: usize,
	pub mime: String,
}

pub enum ExifErrorKind {
	FileOpenError,
	FileSeekError,
	FileReadError,
	FileTypeUnknown,
	JpegWithoutExif,
	TiffTruncated,
	TiffBadPreamble,
	IfdTruncated,
	ExifIfdTruncated,
	ExifIfdEntryNotFound,
}

pub struct ExifError {
	pub kind: ExifErrorKind,
	pub extra: String
}

pub struct IfdEntry {
	pub tag: u16,
	pub format: u16,
	pub count: u32,
	pub data: [u8; 4],
	pub le: bool,
}

impl IfdEntry {
	fn data_as_offset(&self) -> usize {
		read_u32(self.le, &self.data[0..4]) as usize
	}
}

impl ExifError {
	fn readable(&self) -> &str {
		let msg = match self.kind {
			ExifErrorKind::FileOpenError => "File could not be opened",
			ExifErrorKind::FileSeekError => "File could not be seeked",
			ExifErrorKind::FileReadError => "File could not be read",
			ExifErrorKind::FileTypeUnknown => "File type unknown",
			ExifErrorKind::JpegWithoutExif => "JPEG without EXIF section",
			ExifErrorKind::TiffTruncated => "TIFF truncated at start",
			ExifErrorKind::TiffBadPreamble => "TIFF with bad preamble",
			ExifErrorKind::IfdTruncated => "TIFF IFD truncated",
			ExifErrorKind::ExifIfdTruncated => "TIFF Exif IFD truncated",
			ExifErrorKind::ExifIfdEntryNotFound => "TIFF Exif IFD not found",
		};
		return msg;
	}
}

impl Error for ExifError {
	fn description(&self) -> &str {
		self.readable()
	}
}

impl Debug for ExifError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.readable(), self.extra)
	}
}

impl Display for ExifError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.readable(), self.extra)
	}
}

pub type ExifResult = Result<RefCell<ExifData>, ExifError>;

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

pub fn find_embedded_tiff(contents: &Vec<u8>) -> (usize, usize, String)
{
	let mut err = "Scan past EOF and no EXIF found".to_string();
	
	{
	let mut offset = 2 as usize;
	let mut size = 0 as usize;

	while offset < contents.len() {
		if contents.len() < (offset + 4) {
			err = "JPEG truncated in marker header".to_string();
			break;
		}

		let marker: u16 = (contents[offset] as u16) * 256 + (contents[offset + 1] as u16);

		if marker < 0xff00 {
			err = format!("Invalid marker {:x}", marker);
			break;
		}

		offset += 2;
		size = (contents[offset] as usize) * 256 + (contents[offset + 1] as usize);

		if size < 2 {
			err = "JPEG marker size must be at least 2 (because of the size word)".to_string();
			break;
		}
		if contents.len() < (offset + size) {
			err = "JPEG truncated in marker body".to_string();
			break;
		}

		if marker == 0xffe1 {
			// Discard the size word
			offset += 2;
			size -= 2;

			if size < 6 {
				err = "EXIF preamble truncated".to_string();
				break;
			}

			if contents[offset + 0] != ('E' as u8) &&
					contents[offset + 1] != ('x' as u8) &&
					contents[offset + 2] != ('i' as u8) &&
					contents[offset + 3] != ('f' as u8) &&
					contents[offset + 4] != 0 &&
					contents[offset + 5] != 0 {
				err = "EXIF preamble unrecognized".to_string();
				break;
			}

			// Discard the 'Exif\0\0' preamble
			offset += 6;
			size -= 6;

			return (offset, size, "".to_string());
		}
		if marker == 0xffda {
			// last marker
			err = "Last mark found and no EXIF".to_string();
			break;
		}

		offset += size;
	}
	}

	return (0, 0, err);
}

fn read_u16(le: bool, raw: &[u8]) -> u16
{
	if le {
		(raw[1] as u16) * 256 + raw[0] as u16
	} else {
		(raw[0] as u16) * 256 + raw[1] as u16
	}
}

fn read_u32(le: bool, raw: &[u8]) -> u32
{
	if le {
		((raw[3] as u32) << 24) + ((raw[2] as u32) << 16) +
		((raw[1] as u32) << 8) + raw[0] as u32
	} else {
		((raw[0] as u32) << 24) + ((raw[1] as u32) << 16) +
		((raw[2] as u32) << 8) + raw[3] as u32
	}
}

fn parse_ifd(le: bool, count: u16, contents: &[u8]) -> (Vec<IfdEntry>, usize)
{
	let mut entries: Vec<IfdEntry> = Vec::new();

	for i in 0..count {
		// println!("Parsing IFD entry {}", i);
		let mut offset = ((i as usize) * 12);
		let tag = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let format = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let count = read_u32(le, &contents[offset..offset + 4]);
		offset += 4;
		let data = [contents[offset], contents[offset + 1],
			contents[offset + 2], contents[offset + 3]];

		let entry = IfdEntry{tag: tag, format: format, count: count, data: data, le: le};
		entries.push(entry);
	}

	let next_ifd = read_u32(le, &contents[count as usize * 12..]) as usize;

	return (entries, next_ifd);
}

fn parse_exif_ifd_entry(le: bool, contents: &[u8], offset: usize) -> ExifResult
{
	return Ok(RefCell::new(ExifData{file: "".to_string(), size: 0, mime: "".to_string()}));
}

fn parse_ifds(le: bool, first_offset: usize, contents: &[u8]) -> ExifResult
{
	let mut offset = first_offset;

	// FIXME handle circular reference (when some IFD points to a former one)

	while offset != 0 {
		// println!("Offset is {}", offset);
		if contents.len() < (offset + 2) {
			return Err(ExifError{
				kind: ExifErrorKind::IfdTruncated,
				extra: "Truncated at dir entry count".to_string()});
		}

		let count = read_u16(le, &contents[offset..offset + 2]);
		// println!("IFD entry count is {}", count);
		let ifd_length = ((count as usize) * 12 + 4);
		offset += 2;

		if contents.len() < (offset + ifd_length) {
			return Err(ExifError{
				kind: ExifErrorKind::IfdTruncated,
				extra: "Truncated at dir listing".to_string()});
		}

		let (ifd, next_ifd) = parse_ifd(le, count, &contents[offset..offset + ifd_length]);

		for entry in &ifd {
			// println!("Reading tag {:x}", entry.tag);
			if entry.tag == 0x8769 {
				let exif_offset = entry.data_as_offset();

				if contents.len() < exif_offset {
					return Err(ExifError{
						kind: ExifErrorKind::ExifIfdTruncated,
						extra: "Exif IFD goes past EOF".to_string()});
				}

				return parse_exif_ifd_entry(le, &contents, exif_offset);
			}
		}

		offset = next_ifd;

		if offset == 0 {
			// End of IFD chain
			break;
		}
	}

	return Err(ExifError{
			kind: ExifErrorKind::ExifIfdEntryNotFound,
			extra: "".to_string()});
}

pub fn parse_tiff(contents: &[u8]) -> ExifResult
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

pub fn parse_buffer(fname: &str, contents: &Vec<u8>) -> ExifResult
{
	let mime = detect_type(&contents);

	if mime == "" {
		return Err(ExifError{
				kind: ExifErrorKind::FileTypeUnknown,
				extra: "".to_string()});
	}

	let mut offset = 0 as usize;
	let mut size = contents.len() as usize;

	if mime == "image/jpeg" {
		let (eoffset, esize, err) = find_embedded_tiff(&contents);
		if eoffset == 0 {
			return Err(ExifError{
				kind: ExifErrorKind::JpegWithoutExif,
				extra: err.to_string()});
		}
		offset = eoffset;
		size = esize;
		// println!("Offset {} size {}", offset, size);
	}
	match parse_tiff(&contents[offset .. offset + size]) {
		Ok(d) => {
				d.borrow_mut().size = contents.len();
				d.borrow_mut().file = fname.to_string();
				d.borrow_mut().mime = mime.to_string();
				Ok(d)
			},
		Err(e) => Err(e)
	}

}

pub fn read_file(fname: &str, f: &mut File) -> ExifResult
{
	match f.seek(SeekFrom::Start(0)) {
		Ok(_) => (),
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileSeekError,
				extra: fname.to_string()}),
	}

	// TODO: should read only the relevant parts of a file,
	// and pass a StringIO-like object instead of a Vec buffer

	let mut contents: Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Ok(_) => parse_buffer(&fname, &contents),
		Err(_) => Err(ExifError{kind: ExifErrorKind::FileReadError,
				extra: fname.to_string()}),
	}
}

pub fn parse_file(fname: &str) -> ExifResult
{
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileOpenError,
				extra: fname.to_string()}),
	};
	return read_file(fname, &mut f);
}

use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
use std::result::Result;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

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
	JpegWithoutTiff,
}

pub struct ExifError {
	pub kind: ExifErrorKind,
	pub extra: String
}

impl ExifError {
	fn readable(&self) -> &str {
		let msg = match self.kind {
			ExifErrorKind::FileOpenError => "File could not be opened",
			ExifErrorKind::FileSeekError => "File could not be seeked",
			ExifErrorKind::FileReadError => "File could not be read",
			ExifErrorKind::FileTypeUnknown => "File type unknown",
			ExifErrorKind::JpegWithoutTiff => "JPEG without embedded TIFF that contains EXIF",
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

pub type ExifResult = Result<ExifData, ExifError>;

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

pub fn find_embedded_tiff(contents: &Vec<u8>) -> (usize, usize)
{
	let mut offset = 2 as usize;
	let mut size = 0 as usize;
	loop {
		if contents.len() < (offset + 4) {
			println!("JPEG truncated in marker header");
			offset = 0;
			size = 0;
			break;
		}

		let marker: u16 = (contents[offset] as u16) * 256 + (contents[offset + 1] as u16);

		if (marker < 0xff00) {
			println!("Invalid marker {}", marker);
			offset = 0;
			size = 0;
			break;
		}

		offset += 2;
		size = (contents[offset] as usize) * 256 + (contents[offset + 1] as usize);

		if size < 2 {
			println!("JPEG marker size must be at least 2 (because of the size word)");
			offset = 0;
			size = 0;
			break;
		}
		if contents.len() < (offset + size) {
			println!("JPEG truncated in marker body");
			offset = 0;
			size = 0;
			break;
		}

		if marker == 0xffe1 {
			println!("Found Tiff marker");
			// Discard the size word
			offset += 2;
			size -= 2;
			break;
		}
		if marker == 0xffda {
			// last marker
			println!("Last mark found and no EXIF");
			offset = 0;
			size = 0;
			break;
		}

		println!("Jumping marker {}", marker);
		offset += size;
	}

	return (offset, size);
}

pub fn parse_tiff(contents: &Vec<u8>, offset: usize, size: usize) -> ExifResult
{
	return Ok(ExifData{file: "".to_string(), size: 0, mime: "".to_string()});
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
		let (offset, size) = find_embedded_tiff(&contents);
		if offset == 0 {
			return Err(ExifError{
				kind: ExifErrorKind::JpegWithoutTiff,
				extra: "".to_string()});
		}
	}
	match parse_tiff(&contents, offset, size) {
		Ok(d) => {
				/* FIXME
				d.size = contents.len();
				d.file = fname.to_string();
				d.mime = mime.to_string();
				*/
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

	// FIXME: should read only the relevant parts of a file,
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

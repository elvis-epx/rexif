use std::fs::File;
use std::io::{Seek,SeekFrom,Read};

mod lowlevel;
mod rational;
pub use self::rational::*;
mod types;
pub use self::types::*;
mod types_impl;
pub use self::types_impl::*;
mod debug;
mod image;
use self::image::*;
mod ifdformat;
mod tiff;
use self::tiff::*;
mod exifreadable;
mod exifpost;
mod exif;


/// Parse an image buffer that may be of any format. Detect format and find EXIF data
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
		let (eoffset, esize, err) = find_embedded_tiff_in_jpeg(&contents);
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
			let f = ExifData { mime: mime.to_string(), entries: d };
			Ok(f)
		},
		Err(e) => Err(e)
	}
}

/// Read and parse an open file that is supposed to contain an image
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

/// Parse an image file
pub fn parse_file(fname: &str) -> ExifResult
{
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileOpenError,
				extra: fname.to_string()}),
	};
	return read_file(fname, &mut f);
}

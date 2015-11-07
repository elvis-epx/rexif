//! RExif is a native Rust create, written to extract EXIF data from JPEG and TIFF images.
//! 
//! Note that it is in very early stages of development. Any sort of feedback is welcome!
//!
//! The crate contains a
//! sample binary called 'rexiftool' that accepts files as arguments and prints the EXIF data. It gives
//! a rough idea on how to use the crate. Get some sample images and run
//!
//!
//! `cargo run [image file 1] [image file 2] ...`
//!
//!
//! To learn to use this crate, start by the documentation of function `parse_file()`, 
//! and the struct `ExifData` that is returned by the parser. The rest falls more or less into place.
//!
//! Code sample lightly edited from src/bin.rs: 
//!
//! ```
//! match rexif::parse_file(&file_name) {
//!	Ok(exif) => {
//!		println!("{} {} exif entries: {}", file_name,
//!			exif.mime, exif.entries.len());
//!
//!		for entry in &exif.entries {
//!			println!("	{}: {}",
//!					entry.tag_readable, 
//!					entry.value_more_readable);
//!		}
//!	},
//!	Err(e) => {
//!		print!("Error in {}: {} {}", &file_name,
//!			Error::description(&e), e.extra).unwrap();
//!	}
//! }
//! ```

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
mod nikon;

/// Parse a byte buffer that should contain a TIFF or JPEG image.
/// Tries to detect format and parse EXIF data.
pub fn parse_buffer(contents: &Vec<u8>) -> ExifResult
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

/// Try to read and parse an open file that is expected to contain an image
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
		Ok(_) => parse_buffer(&contents),
		Err(_) => Err(ExifError{kind: ExifErrorKind::FileReadError,
				extra: fname.to_string()}),
	}
}

/// Opens an image (passed as a file name), tries to read and parse it.
pub fn parse_file(fname: &str) -> ExifResult
{
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileOpenError,
				extra: fname.to_string()}),
	};
	return read_file(fname, &mut f);
}

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
//! use std::error::Error;
//! let file_name = "foo.jpg";
//! match rexif::parse_file(&file_name) {
//!	Ok(exif) => {
//!		println!("{} {} exif entries: {}", file_name,
//!			exif.mime, exif.entries.len());
//!
//!		for entry in &exif.entries {
//!			println!("	{}: {}",
//!					entry.tag,
//!					entry.value_more_readable);
//!		}
//!	},
//!	Err(e) => {
//!		print!("Error in {}: {}", &file_name, e)
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

/// Parse a byte buffer that should contain a TIFF or JPEG image.
/// Tries to detect format and parse EXIF data.
pub fn parse_buffer(contents: &[u8]) -> ExifResult
{
	let mime = detect_type(contents);

	let d = match mime {
		"" => return Err(ExifError::FileTypeUnknown),
		"image/jpeg" => {
			let (offset, size) = try!(find_embedded_tiff_in_jpeg(contents));
			// println!("Offset {} size {}", offset, size);
			try!(parse_tiff(&contents[offset .. offset + size]))
		},
		_ => {
			try!(parse_tiff(&contents))
		}
	};

	Ok(ExifData {
		mime: mime.to_string(),
		entries: d,
	})
}

/// Try to read and parse an open file that is expected to contain an image
pub fn read_file(f: &mut File) -> ExifResult
{
	try!(f.seek(SeekFrom::Start(0)));

	// TODO: should read only the relevant parts of a file,
	// and pass a StringIO-like object instead of a Vec buffer

	let mut contents: Vec<u8> = Vec::new();
	try!(f.read_to_end(&mut contents));
	parse_buffer(&contents)
}

/// Opens an image (passed as a file name), tries to read and parse it.
pub fn parse_file(fname: &str) -> ExifResult
{
	read_file(&mut try!(File::open(fname)))
}

use std::fs::File;
use std::io::{Result,Seek,SeekFrom,Read};

pub struct ExifData {
	pub file: String,
	pub size: usize,
}

pub fn parse_buffer(fname: &str, contents: &Vec<u8>) -> ExifData
{
	return ExifData{file: fname.to_string(), size: contents.len()};
}

pub fn read_file(fname: &str, f: &mut File) -> Result<ExifData>
{
	match f.seek(SeekFrom::Start(0)) {
		Ok(_) => (),
		Err(e) => return Err(e),
	}

	// FIXME: should read only the relevant parts of a file,
	// and pass a StringIO-like object instead of a Vec buffer

	let mut contents: Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Ok(_) => Ok(parse_buffer(&fname, &contents)),
		Err(e) => Err(e),
	}
}

pub fn parse_file(fname: &str) -> Result<ExifData>
{
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(e) => return Err(e)
	};
	return read_file(fname, &mut f);
}

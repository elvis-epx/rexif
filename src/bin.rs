use std::env;
use std::process;
use std::io::Write;
use std::error::Error;
extern crate rexif;

fn main()
{
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		writeln!(std::io::stderr(), "Usage: {} image1 image2 ...", args[0]);
		process::exit(2);
	}
	for arg in &args[1..] {
		match rexif::parse_file(&arg) {
			Ok(exif) => {
				let exif = exif.into_inner();
				println!("{} {} {}", exif.file, exif.size, exif.mime);
			},
			Err(e) => {
				writeln!(std::io::stderr(), "Error in {}: {} {}", &arg, Error::description(&e), e.extra);
			},
	 	}
	}
}

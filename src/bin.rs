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
	for arg in &args {
		match rexif::parse_file(&arg) {
			Ok(exif) => {
				println!("{} {}", exif.size, exif.file);
			},
			Err(e) => {
				writeln!(std::io::stderr(), "{}: {}", &arg, Error::description(&e));
			},
	 	}
	}
}

use std::env;
use std::process;
use std::io::Write;
extern crate rexif;

use rexif::IfdTag;

/// Tries to extract EXIF data from all files passed as CLI parameters,
/// assuming that the files contain images.
fn main()
{
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		writeln!(std::io::stderr(), "Usage: {} image1 image2 ...", args[0]).unwrap();
		process::exit(2);
	}
	for arg in &args[1..] {
		match rexif::parse_file(&arg) {
			Ok(exif) => {
				println!("{} {} exif entries: {}",
					arg, exif.mime, exif.entries.len());
				for entry in &exif.entries {
					match entry.tag {
						IfdTag::Unknown(_) => {},
						IfdTag::Exif(tag) => {
							println!("	{}: {}",
									tag,
									entry.value_more_readable);
						},
					}
				}
			},
			Err(e) => {
				writeln!(std::io::stderr(), "Error in {}: {}", &arg, e).unwrap();
			}
	 	}
	}
}

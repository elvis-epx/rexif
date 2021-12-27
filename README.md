# rexif

# NOTE: THIS VERSION IS OBSOLETE

The current maintainer repository of this library is https://github.com/kornelski/rexif.

# Old documentation

RExif is a native Rust crate, written to extract EXIF data from JPEG and TIFF images.

It is in very early stages of development. Documentation and examples are still scarce,
sorry about that. It is also my pet project to learn a bit of Rust. It is very fast:
it takes about 2ms to parse an in-memory image (measured on Core i5).

The crate also contains a
sample binary called 'rexiftool' that accepts files as arguments and prints the EXIF data. It gives
a rough idea on how to use the crate.

I am still filling in
the implementation of most EXIF tags. Merge requests, comments and criticisms about coding style, information
about uncovered EXIF tags, sample images that are not parsed correctly -- in short, any sort of feedback is
welcome!

# Example

```
match rexif::parse_file(&file_name) {
	Ok(exif) => {
		println!("{} {} exif entries: {}", file_name,
			exif.mime, exif.entries.len());

		for entry in &exif.entries {
			println!("	{}: {}",
					entry.tag_readable, 
					entry.value_more_readable);
		}
	},
	Err(e) => {
		print!("Error in {}: {} {}", &file_name,
			Error::description(&e), e.extra).unwrap();
	}
}
```

The included tool `refixtool` accepts image file names as command-line
parameters and prints EXIF data for them. The `src/main.rs` file is a
good starting point to learn how to use the crate, then take a look into
the `ExifEntry` struct.

# Contact

Elvis Pfützenreuter - epxx@epxx.co - https://epxx.co

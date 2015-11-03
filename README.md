# rexif

RExif is a native Rust library to extract EXIF data from JPEG and TIFF images.

It is in very early stages of development. Documentation and examples are still scarce.

The crate also contains a
sample binary called 'rexiftoolt' that accepts files as arguments and prints the EXIF data. It gives
a rough idea on how to use the crate.

I am still filling in
the implementation of most EXIF tags. Merge requests, comments and criticisms about coding style, information
about uncovered EXIF tags, sample images that are not parsed correctly -- in short, any sort of feedback is
welcome!

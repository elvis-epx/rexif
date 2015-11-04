use std::io;
use std::io::Write;

pub fn warning(msg: &str) {
	match io::stderr().write(msg.as_bytes()) {
		_ => (),
	};
	match io::stderr().write("\n".as_bytes()) {
		_ => (),
	};
}

use std::io;
use std::io::Write;

/// Print message to stderr, fails silently if not possible
pub fn warning(msg: &str) {
	match io::stderr().write(msg.as_bytes()) {
		_ => (),
	};
	match io::stderr().write("\n".as_bytes()) {
		_ => (),
	};
}

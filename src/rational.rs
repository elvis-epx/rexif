use std::fmt::Formatter;
use std::fmt;
use std::fmt::Display;

/// Type used by TIFF that represents a rational number
#[derive(Copy, Clone)]
pub struct IRational {
	pub numerator: i32,
	pub denominator: i32,
}

impl IRational {
	/// Floating point value (numerator divided by denominator)
	pub fn value(&self) -> f64 {
		(self.numerator as f64) / (self.denominator as f64)
	}
}

impl Display for IRational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}

#[derive(Copy, Clone)]
/// Type used by TIFF that represents a rational number
pub struct URational {
	pub numerator: u32,
	pub denominator: u32,
}

impl URational {
	/// Floating point value (numerator divided by denominator)
	pub fn value(&self) -> f64 {
		(self.numerator as f64) / (self.denominator as f64)
	}
}

impl Display for URational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}


use std::fmt;
use std::fmt::Display;

/// Encapsulation of the TIFF type that represents a signed rational number
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
/// Encapsulation of the TIFF type that represents an unsigned rational number
pub struct URational {
	pub numerator: u32,
	pub denominator: u32,
}

impl URational {
	/// Floating point value (numerator divided by denominator)
	pub fn value(&self) -> f64 {
		f64::from(self.numerator) / f64::from(self.denominator)
	}
}

impl Display for URational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}


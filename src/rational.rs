use std::fmt::Formatter;
use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct IRational {
	pub numerator: i32,
	pub denominator: i32,
}

impl IRational {
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
pub struct URational {
	pub numerator: u32,
	pub denominator: u32,
}

impl URational {
	pub fn value(&self) -> f64 {
		(self.numerator as f64) / (self.denominator as f64)
	}
}

impl Display for URational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}


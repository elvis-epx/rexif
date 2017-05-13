use std::fmt::Display;
use itertools::Itertools;

pub trait ToCsv<T> {
	fn to_csv(&self) -> String;
}

impl<T: Display> ToCsv<T> for Vec<T> {
	fn to_csv(&self) -> String {
		self.iter().join(", ")
	}
}

impl<T: Display> ToCsv<T> for [T] {
	fn to_csv(&self) -> String {
		self.iter().join(", ")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn to_csv_should_comma_space_separate_elements() {
		let vec = vec![0, 1, 2, 3];

		assert_eq!("0, 1, 2, 3", vec.to_csv());
	}

	#[test]
	fn to_csv_should_return_empty_string_for_empty_vector() {
		let vec: Vec<u8> = vec![];

		assert_eq!("", vec.to_csv());
	}

	#[test]
	fn to_csv_should_return_a_one_element_vector_as_that_element_stringified() {
		let vec = vec![5];

		assert_eq!("5", vec.to_csv());
	}

	#[test]
	fn to_csv_should_comma_space_separate_elements_of_a_slice() {
		let vec = vec![0, 1, 2, 3];

		assert_eq!("0, 1, 2, 3", vec[..].to_csv());
	}
}

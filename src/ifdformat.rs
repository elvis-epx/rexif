use std::fmt::Display;
use itertools::Itertools;
use super::types::*;
use super::lowlevel::*;

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

/// Convert a IfdEntry into a tuple of TagValue
pub fn tag_value_new(f: &IfdEntry) -> TagValue
{
	match f.format {
		IfdFormat::Ascii => {
			// Remove \0, there may be more than one
			let mut tot = f.data.len();
			while tot > 0 && f.data[tot - 1] == 0 {
				tot -= 1;
			}
			// In theory it should be pure ASCII but we admit UTF-8
			let s = String::from_utf8_lossy(&f.data[0..tot]);
			let s = s.into_owned();
			TagValue::Ascii(s.to_string())
		},
		IfdFormat::U16 => {
			if f.data.len() < (f.count as usize * 2) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_u16_array(f.le, f.count, &f.data[..]);
			TagValue::U16(a)
		},
		IfdFormat::I16 => {
			if f.data.len() < (f.count as usize * 2) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_i16_array(f.le, f.count, &f.data[..]);
			TagValue::I16(a)
		},
		IfdFormat::U8 => {
			if f.data.len() < (f.count as usize * 1) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = f.data.clone();
			TagValue::U8(a)
		},
		IfdFormat::I8 => {
			if f.data.len() < (f.count as usize * 1) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_i8_array(f.count, &f.data[..]);
			TagValue::I8(a)
		},
		IfdFormat::U32 => {
			if f.data.len() < (f.count as usize * 4) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_u32_array(f.le, f.count, &f.data[..]);
			TagValue::U32(a)
		},
		IfdFormat::I32 => {
			if f.data.len() < (f.count as usize * 4) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_i32_array(f.le, f.count, &f.data[..]);
			TagValue::I32(a)
		},
		IfdFormat::F32 => {
			if f.data.len() < (f.count as usize * 4) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_f32_array(f.count, &f.data[..]);
			TagValue::F32(a)
		},
		IfdFormat::F64 => {
			if f.data.len() < (f.count as usize * 8) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_f64_array(f.count, &f.data[..]);
			TagValue::F64(a)
		},
		IfdFormat::URational => {
			if f.data.len() < (f.count as usize * 8) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_urational_array(f.le, f.count, &f.data[..]);
			TagValue::URational(a)
		},
		IfdFormat::IRational => {
			if f.data.len() < (f.count as usize * 8) {
				return TagValue::Invalid(f.data.clone(), f.le,
							             f.format, f.count);
			}
			let a = read_irational_array(f.le, f.count, &f.data[..]);
			TagValue::IRational(a)
		},

		IfdFormat::Undefined => {
			let a = f.data.clone();
			TagValue::Undefined(a, f.le)
		},

		_ => TagValue::Unknown(f.data.clone(), f.le)
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

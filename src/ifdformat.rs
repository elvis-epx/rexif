use std::fmt::Display;
use super::types::*;
use super::lowlevel::*;

/// generic function that prints a string representation of a vector
pub fn numarray_to_string<T: Display>(numbers: &Vec<T>) -> String
{
	if numbers.len() < 1 {
		return "".to_string();
	} else if numbers.len() == 1 {
		return format!("{}", &numbers[0]);
	}

	let mut s = "".to_string();
	let mut first = true;
	for number in numbers {
		if !first {
			s = s + ", ";
		}
		first = false;
		let s2 = format!("{}", number);
		s = s + &s2;
	}

	return s;
}

/// Convert a IfdEntry into a tuple of TagValue and a crude string representation of tag value
pub fn tag_value_new(f: &IfdEntry) -> (TagValue, String)
{
	match f.format {
		IfdFormat::Str => {
			let s = String::from_utf8_lossy(&f.data[..]);
			let s = s.into_owned();
			(TagValue::Str(s.to_string()), s.to_string())
		},
		IfdFormat::U16 => {
			let a = read_u16_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::U16(a), b)
		},
		IfdFormat::I16 => {
			let a = read_i16_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I16(a), b)
		},
		IfdFormat::U8 => {
			let a = f.data.clone();
			let b = numarray_to_string(&a);
			(TagValue::U8(a), b)
		},
		IfdFormat::I8 => {
			let a = read_i8_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I8(a), b)
		},
		IfdFormat::U32 => {
			let a = read_u32_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::U32(a), b)
		},
		IfdFormat::I32 => {
			let a = read_i32_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I32(a), b)
		},
		IfdFormat::F32 => {
			let a = read_f32_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::F32(a), b)
		},
		IfdFormat::F64 => {
			let a = read_f64_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::F64(a), b)
		},
		IfdFormat::URational => {
			let a = read_urational_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::URational(a), b)
		},
		IfdFormat::IRational => {
			let a = read_irational_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::IRational(a), b)
		},

		IfdFormat::Undefined => {
			let a = f.data.clone();
			let b = numarray_to_string(&a);
			(TagValue::Undefined(a), b)
		},

		_ => (TagValue::Unknown(f.data.clone()),
					"<unknown blob>".to_string()),
	}
}


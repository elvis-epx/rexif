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
		IfdFormat::Ascii => {
			// Remove \0, there may be more than one
			let mut tot = f.data.len();
			while tot > 0 && f.data[tot - 1] == 0 {
				tot -= 1;
			}
			// In theory it should be pure ASCII but we admit UTF-8
			let s = String::from_utf8_lossy(&f.data[0..tot]);
			let s = s.into_owned();
			(TagValue::Ascii(s.to_string()), s.to_string())
		},
		IfdFormat::U16 => {
			if f.data.len() < (f.count as usize * 2) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_u16_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::U16(a), b)
		},
		IfdFormat::I16 => {
			if f.data.len() < (f.count as usize * 2) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_i16_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I16(a), b)
		},
		IfdFormat::U8 => {
			if f.data.len() < (f.count as usize * 1) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = f.data.clone();
			let b = numarray_to_string(&a);
			(TagValue::U8(a), b)
		},
		IfdFormat::I8 => {
			if f.data.len() < (f.count as usize * 1) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_i8_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I8(a), b)
		},
		IfdFormat::U32 => {
			if f.data.len() < (f.count as usize * 4) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_u32_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::U32(a), b)
		},
		IfdFormat::I32 => {
			if f.data.len() < (f.count as usize * 4) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_i32_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I32(a), b)
		},
		IfdFormat::F32 => {
			if f.data.len() < (f.count as usize * 4) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_f32_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::F32(a), b)
		},
		IfdFormat::F64 => {
			if f.data.len() < (f.count as usize * 8) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_f64_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::F64(a), b)
		},
		IfdFormat::URational => {
			if f.data.len() < (f.count as usize * 8) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_urational_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::URational(a), b)
		},
		IfdFormat::IRational => {
			if f.data.len() < (f.count as usize * 8) {
				return (TagValue::Invalid(f.data.clone(), f.le,
							f.format as u16, f.count),
					"Invalid".to_string());
			}
			let a = read_irational_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::IRational(a), b)
		},

		IfdFormat::Undefined => {
			let a = f.data.clone();
			let b = numarray_to_string(&a);
			(TagValue::Undefined(a, f.le), b)
		},

		_ => (TagValue::Unknown(f.data.clone(), f.le),
					"<unknown blob>".to_string()),
	}
}


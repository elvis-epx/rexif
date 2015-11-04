use super::types::*;
use super::ifdformat::*;

/// No-op for readable value tag function
pub fn nop(_: &TagValue, s: &String) -> String
{
	return s.clone();
}

/// No-op for readable value tag function that should be kept as simple strings
pub fn strpass(_: &TagValue, s: &String) -> String
{
	return s.clone();
}

pub fn orientation(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Straight",
				3 => "Upside down",
				6 => "Rotated to left",
				8 => "Rotated to right",
				9 => "Undefined",
				_ => "(Invalid)",
			}
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

pub fn rational_value(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{}", v[0].value())
		},
		&TagValue::IRational(ref v) => {
			format!("{}", v[0].value())
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

pub fn rational_values(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			let ve: Vec<f64> = v.iter().map(|&x| x.value()).collect();
			numarray_to_string(&ve)
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

pub fn resolution_unit(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Unitless",
				2 => "in",
				3 => "cm",
				_ => "(Invalid)",
			}
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

pub fn exposure_time(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{} s", v[0])
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

pub fn f_number(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("f/{:1}", v[0].value())
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}


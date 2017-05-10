use itertools::Itertools;
use super::types::*;
use super::ifdformat::*;
use super::lowlevel::read_u16_array;

static INV: &'static str = "Invalid data for this tag";

/// No-op for readable value tag function. Used for ASCII string tags, or when the
/// default readable representation of value is pretty enough.
pub fn strpass(e: &TagValue) -> String
{
	format!("{}", e)
}

pub fn orientation(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			1 => "Straight",
			3 => "Upside down",
			6 => "Rotated to left",
			8 => "Rotated to right",
			9 => "Undefined",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn rational_value(e: &TagValue) -> String
{
	match e {
		&TagValue::URational(ref v) => {
			v.iter().map(|x| x.value()).join(", ")
		},
		&TagValue::IRational(ref v) => {
			v.iter().map(|x| x.value()).join(", ")
		},
		_ => panic!(INV),
	}
}

pub fn resolution_unit(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			1 => "Unitless",
			2 => "in",
			3 => "cm",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn exposure_time(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		let r = v[0];
		if r.numerator == 1 && r.denominator > 1 {
			// traditional 1/x exposure time
			format!("{} s", r)
		} else if r.value() < 0.1 {
			format!("1/{:.0} s", 1.0 / r.value())
		} else if r.value() < 1.0 {
			format!("1/{:.1} s", 1.0 / r.value())
		} else {
			format!("{:.1} s", r.value())
		}
	} else {
		panic!(INV)
	}
}

pub fn f_number(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("f/{:.1}", v[0].value())
	} else {
		panic!(INV)
	}
}

pub fn exposure_program(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			1 => "Manual control",
			2 => "Program control",
			3 => "Aperture priority",
			4 => "Shutter priority",
			5 => "Program creative (slow program)",
			6 => "Program creative (high-speed program)",
			7 => "Portrait mode",
			8 => "Landscape mode",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn focal_length(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{} mm", v[0].value())
	} else {
   		panic!(INV)
   	}
}

pub fn focal_length_35(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		format!("{} mm", v[0])
	} else {
   		panic!(INV)
   	}
}

pub fn meters(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{:.1} m", v[0].value())
	} else {
   		panic!(INV)
   	}
}

pub fn iso_speeds(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		if v.len() == 1 {
			format!("ISO {}", v[0])
		} else if v.len() == 2 || v.len() == 3 {
			format!("ISO {} latitude {}", v[0], v[1])
		} else {
			format!("Unknown ({})", v.to_csv())
		}
	} else {
   		panic!(INV)
   	}
}

pub fn dms(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		let deg = v[0];
		let min = v[1];
		let sec = v[2];
		if deg.denominator == 1 && min.denominator == 1 {
			format!("{}°{}'{:.2}\"", deg.value(), min.value(), sec.value())
		} else if deg.denominator == 1 {
			format!("{}°{:.4}'", deg.value(), min.value() + sec.value() / 60.0)
		} else {
			// untypical format
			format!("{:.7}°", deg.value() + min.value() / 60.0 + sec.value() / 3600.0)
		}
	} else {
   		panic!(INV)
   	}
}

pub fn gps_alt_ref(e: &TagValue) -> String
{
	if let &TagValue::U8(ref v) = e {
		match v[0] {
			0 => "Above sea level",
			1 => "Below sea level",
			n @ _ => return format!("Unknown, assumed below sea level ({})", n),
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

pub fn gpsdestdistanceref(e: &TagValue) -> String
{
	if let &TagValue::Ascii(ref v) = e {
		match v.as_str() {
			"N" => "kn",
			"K" => "km",
			"M" => "mi",
			n @ _ => return format!("Unknown ({})", n)
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

pub fn gpsdestdistance(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{:.3}", v[0].value())
	} else {
   		panic!(INV)
   	}
}

pub fn gpsspeedref(e: &TagValue) -> String
{
	if let &TagValue::Ascii(ref v) = e {
		match v.as_str() {
			"N" => "kn",
			"K" => "km/h",
			"M" => "mi/h",
			n @ _ => return format!("Unknown ({})", n)
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

pub fn gpsspeed(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{:.1}", v[0].value())
	} else {
   		panic!(INV)
   	}
}

pub fn gpsbearingref(e: &TagValue) -> String
{
	if let &TagValue::Ascii(ref v) = e {
		match v.as_str() {
			"T" => "True bearing",
			"M" => "Magnetic bearing",
			n @ _ => return format!("Unknown ({})", n)
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

pub fn gpsbearing(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{:.2}°", v[0].value())
	} else {
   		panic!(INV)
   	}
}

pub fn gpstimestamp(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		let hour = v[0];
		let min = v[1];
		let sec = v[2];
		format!("{:02.0}:{:02.0}:{:04.1} UTC", hour.value(), min.value(), sec.value())
	} else {
   		panic!(INV)
   	}
}

pub fn gpsdiff(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Measurement without differential correction",
			1 => "Differential correction applied",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

pub fn gpsstatus(e: &TagValue) -> String
{
	if let &TagValue::Ascii(ref v) = e {
		match v.as_str() {
			"A" => "Measurement in progress",
			"V" => "Measurement is interoperability",
			n @ _ => return format!("Unknown ({})", n)
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

pub fn gpsmeasuremode(e: &TagValue) -> String
{
	if let &TagValue::Ascii(ref v) = e {
		match v.as_str() {
			"2" => "2-dimension",
			"3" => "3-dimension",
			n @ _ => return format!("Unknown ({})", n)
		}.to_string()
	} else {
   		panic!(INV)
   	}
}

/// Interprets an Undefined tag as ASCII, when the contents are guaranteed
/// by EXIF standard to be ASCII-compatible. This function accepts UTF-8
/// strings, should they be accepted by EXIF standard in the future.
pub fn undefined_as_ascii(e: &TagValue) -> String
{
	if let &TagValue::Undefined(ref v, _) = e {
		String::from_utf8_lossy(&v[..]).to_string()
	} else {
		panic!(INV)
	}
}

/// Outputs an Undefined tag as an array of bytes. Appropriate for tags
/// that are opaque and small-sized
pub fn undefined_as_u8(e: &TagValue) -> String
{
	if let &TagValue::Undefined(ref v, _) = e {
		v.to_csv()
	} else {
		panic!(INV)
	}
}

/// Tries to parse an Undefined tag as containing a string. For some tags,
/// the string encoding /// format can be discovered by looking into the first
/// 8 bytes.
pub fn undefined_as_encoded_string(e: &TagValue) -> String
{
	// "ASCII\0\0\0"
	static ASC: [u8; 8] = [0x41, 0x53, 0x43, 0x49, 0x49, 0, 0, 0];
	// "JIS\0\0\0\0\0"
	static JIS: [u8; 8] = [0x4a, 0x49, 0x53, 0, 0, 0, 0, 0];
	// "UNICODE\0"
	static UNICODE: [u8; 8] = [0x55, 0x4e, 0x49, 0x43, 0x4f, 0x44, 0x45, 0x00];

	if let &TagValue::Undefined(ref v, le) = e {
		if v.len() < 8 {
			format!("String w/ truncated preamble {}", v.to_csv())
		} else if v[0..8] == ASC[..] {
			let v8 = &v[8..];
			let s = String::from_utf8_lossy(v8);
			s.into_owned()
		} else if v[0..8] == JIS[..] {
			format!("JIS string {}", v[8..].to_csv())
		} else if v[0..8] == UNICODE[..] {
			let v8 = &v[8..];
			// reinterpret as vector of u16
			let v16_size = (v8.len() / 2) as u32;
			let v16 = read_u16_array(le, v16_size, v8);
			String::from_utf16_lossy(&v16)
		} else {
			format!("String w/ undefined encoding {}", v.to_csv())
		}
	} else {
		panic!(INV)
	}
}

/// Prints an opaque and long Undefined tag simply as as "blob", noting its length
pub fn undefined_as_blob(e: &TagValue) -> String
{
	if let &TagValue::Undefined(ref v, _) = e {
		format!("Blob of {} bytes", v.len())
	} else {
		panic!(INV)
	}
}

pub fn apex_tv(e: &TagValue) -> String
{
	if let &TagValue::IRational(ref v) = e {
		format!("{:.1} Tv APEX", v[0].value())
	} else {
		panic!(INV)
	}
}

pub fn apex_av(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{:.1} Av APEX", v[0].value())
	} else {
		panic!(INV)
	}
}

pub fn apex_brightness(e: &TagValue) -> String
{
	if let &TagValue::IRational(ref v) = e {
		// numerator 0xffffffff = unknown
		if v[0].numerator == -1 {
			"Unknown".to_string()
		} else {
			format!("{:.1} APEX", v[0].value())
		}
	} else {
		panic!(INV)
	}
}

pub fn apex_ev(e: &TagValue) -> String
{
	if let &TagValue::IRational(ref v) = e {
		format!("{:.2} EV APEX", v[0].value())
	} else {
		panic!(INV)
	}
}

pub fn file_source(e: &TagValue) -> String
{
    if let &TagValue::Undefined(ref v, _) = e {
		if v.len() > 0 && v[0] == 3 {
			"DSC"
		} else {
			"Unknown"
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn flash_energy(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		format!("{} BCPS", v[0].value())
	} else {
		panic!(INV)
	}
}

pub fn metering_mode(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Unknown",
			1 => "Average",
			2 => "Center-weighted average",
			3 => "Spot",
			4 => "Multi-spot",
			5 => "Pattern",
			6 => "Partial",
			255 => "Other",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn light_source(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Unknown",
			1 => "Daylight",
			2 => "Fluorescent",
			3 => "Tungsten",
			4 => "Flash",
			9 => "Fine weather",
			10 => "Cloudy weather",
			11 => "Shade",
			12 => "Daylight fluorescent (D)",
			13 => "Day white fluorescent (N)",
			14 => "Cool white fluorescent (W)",
			15 => "White fluorescent (WW)",
			17 => "Standard light A",
			18 => "Standard light B",
			19 => "Standard light C",
			20 => "D55",
			21 => "D65",
			22 => "D75",
			23 => "D50",
			24 => "ISO studio tungsten",
			255 => "Other",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn color_space(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			1 => "sRGB",
			65535 => "Uncalibrated",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn flash(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		let n = v[0];
		let mut b0 = "Did not fire. ";
		let mut b12 = "";
		let mut b34 = "";
		let mut b6 = "";

		if (n & (1 << 5)) > 0 {
			return format!("Does not have a flash.");
		}

		if (n & 1) > 0 {
			b0 = "Fired. ";
			if (n & (1 << 6)) > 0 {
				b6 = "Redeye reduction. "
			} else {
				b6 = "No redeye reduction. "
			}

			// bits 1 and 2
			let m = (n >> 1) & 3;
			if m == 2 {
				b12 = "Strobe ret not detected. ";
			} else if m == 3 {
				b12 = "Strobe ret detected. ";
			}
		}

		// bits 3 and 4
		let m = (n >> 3) & 3;
		if m == 1 {
			b34 = "Forced fire. ";
		} else if m == 2 {
			b34 = "Forced suppresion. ";
		} else if m == 3 {
			b12 = "Auto mode. ";
		}

		format!("{}{}{}{}", b0, b12, b34, b6)
	} else {
		panic!(INV)
	}
}

pub fn subject_area(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v.len() {
			2 => format!("at pixel {},{}", v[0], v[1]),
			3 => format!("at center {},{} radius {}", v[0], v[1], v[2]),
			4 => format!("at rectangle {},{} width {} height {}", v[0], v[1], v[2], v[3]),
			_ => format!("Unknown ({}) ", v.to_csv()),
		}
	} else {
		panic!(INV)
	}
}

pub fn subject_location(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		format!("at pixel {},{}", v[0], v[1])
	} else {
		panic!(INV)
	}
}

pub fn sharpness(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Normal",
			1 => "Soft",
			2 => "Hard",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn saturation(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Normal",
			1 => "Low",
			2 => "High",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn contrast(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Normal",
			1 => "Soft",
			2 => "Hard",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn gain_control(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "None",
			1 => "Low gain up",
			2 => "High gain up",
			3 => "Low gain down",
			4 => "High gain down",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn exposure_mode(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Auto exposure",
			1 => "Manual exposure",
			2 => "Auto bracket",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn scene_capture_type(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Standard",
			1 => "Landscape",
			2 => "Portrait",
			3 => "Night scene",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn scene_type(e: &TagValue) -> String
{
	if let &TagValue::Undefined(ref v, _) = e {
		match v[0] {
			1 => "Directly photographed image",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn white_balance_mode(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Auto",
			1 => "Manual",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn sensing_method(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			1 => "Not defined",
			2 => "One-chip color area sensor",
			3 => "Two-chip color area sensor",
			4 => "Three-chip color area sensor",
			5 => "Color sequential area sensor",
			7 => "Trilinear sensor",
			8 => "Color sequential linear sensor",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn custom_rendered(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Normal",
			1 => "Custom",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn subject_distance_range(e: &TagValue) -> String
{
	if let &TagValue::U16(ref v) = e {
		match v[0] {
			0 => "Unknown",
			1 => "Macro",
			2 => "Close view",
			3 => "Distant view",
			n @ _ => return format!("Unknown ({})", n),
		}.to_string()
	} else {
		panic!(INV)
	}
}

pub fn lens_spec(e: &TagValue) -> String
{
	if let &TagValue::URational(ref v) = e {
		let f0 = v[0].value();
		let f1 = v[1].value();
		let a0 = v[2].value();
		let a1 = v[3].value();

		if v[0] == v[1] {
			if a0 == a0 {
				format!("{} mm f/{:.1}", f0, a0)
			} else {
				format!("{} mm f/unknown", f0)
			}
		} else {
			if a0 == a0 && a1 == a1 {
				format!("{}-{} mm f/{:.1}-{:.1}", f0, f1, a0, a1)
			} else {
				format!("{}-{} mm f/unknown", f0, f1)
			}
		}
	} else {
		panic!(INV)
	}
}

#[cfg(test)]
mod tests {
	use rational::IRational;
	use rational::URational;
	use super::*;

	#[test]
	fn rational_value_should_return_a_single_element_vector_as_that_element_stringified() {
		let tag = TagValue::URational(vec![
			URational { numerator: 42, denominator: 7},
		]);
		let string = rational_value(&tag);

		assert_eq!("6", string);

		let tag = TagValue::IRational(vec![
			IRational { numerator: -42, denominator: 7},
		]);
		let string = rational_value(&tag);

		assert_eq!("-6", string);
	}

	#[test]
	fn rational_value_should_return_comma_separated_list_of_values() {
		let tag = TagValue::URational(vec![
			URational { numerator: 1, denominator: 3},
			URational { numerator: 42, denominator: 7},
		]);
		let string = rational_value(&tag);

		assert_eq!("0.3333333333333333, 6", string);
	}

	#[test]
	fn undefined_as_encoded_string_should_return_a_jis_string_as_a_csv_list_of_byte_values() {
		let tag = TagValue::Undefined(vec![0x4a, 0x49, 0x53, 0, 0, 0, 0, 0, 56, 32, 91, 33], true);
		let string = undefined_as_encoded_string(&tag);

		assert_eq!("JIS string 56, 32, 91, 33", string);
	}
}

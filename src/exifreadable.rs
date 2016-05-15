use super::types::*;
use super::ifdformat::*;
use super::lowlevel::read_u16_array;

static INV: &'static str = "Invalid data for this tag";

/// No-op for readable value tag function. Should not be used by any EXIF tag descriptor,
/// except for the catch-all match that handles unknown tags
pub fn nop(e: &TagValue) -> String
{
	format!("{}", e)
}

/// No-op for readable value tag function. Used for ASCII string tags, or when the
/// default readable representation of value is pretty enough.
pub fn strpass(e: &TagValue) -> String
{
	format!("{}", e)
}

pub fn orientation(e: &TagValue) -> String
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
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn rational_value(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{}", v[0].value())
		},
		&TagValue::IRational(ref v) => {
			format!("{}", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn rational_values(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			let ve: Vec<f64> = v.iter().map(|&x| x.value()).collect();
			numarray_to_string(&ve)
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn resolution_unit(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Unitless",
				2 => "in",
				3 => "cm",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn exposure_time(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
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
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn f_number(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("f/{:.1}", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn exposure_program(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Manual control",
				2 => "Program control",
				3 => "Aperture priority",
				4 => "Shutter priority",
				5 => "Program creative (slow program)",
				6 => "Program creative (high-speed program)",
				7 => "Portrait mode",
				8 => "Landscape mode",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn focal_length(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{} mm", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn focal_length_35(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			format!("{} mm", v[0])
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn meters(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{:.1} m", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn iso_speeds(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::U16(ref v) => {
		if v.len() == 1 {
			format!("ISO {}", v[0])
		} else if v.len() == 2 || v.len() == 3 {
			format!("ISO {} latitude {}", v[0], v[1])
		} else {
			format!("Unknown ({})", numarray_to_string(&v))
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn dms(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::URational(ref v) => {
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
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gps_alt_ref(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U8(ref v) => {
			let n = v[0];
			match n {
				0 => "Above sea level",
				1 => "Below sea level",
				_ => return format!("Unknown, assumed below sea level ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsdestdistanceref(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Ascii(ref v) => {
		if v == "N" {
			"kn"
		} else if v == "K" {
			"km"
		} else if v == "M" {
			"mi"
		} else {
			return format!("Unknown ({})", v)
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsdestdistance(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{:.3}", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsspeedref(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Ascii(ref v) => {
		if v == "N" {
			"kn"
		} else if v == "K" {
			"km/h"
		} else if v == "M" {
			"mi/h"
		} else {
			return format!("Unknown ({})", v)
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsspeed(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{:.1}", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsbearingref(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Ascii(ref v) => {
		if v == "T" {
			"True bearing"
		} else if v == "M" {
			"Magnetic bearing"
		} else {
			return format!("Unknown ({})", v)
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsbearing(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{:.2}°", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpstimestamp(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::URational(ref v) => {
		let hour = v[0];
		let min = v[1];
		let sec = v[2];
		format!("{:02.0}:{:02.0}:{:04.1} UTC", hour.value(), min.value(), sec.value())
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsdiff(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Measurement without differential correction",
				1 => "Differential correction applied",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsstatus(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Ascii(ref v) => {
		if v == "A" {
			"Measurement in progress"
		} else if v == "V" {
			"Measurement is interoperability"
		} else {
			return format!("Unknown ({})", v)
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gpsmeasuremode(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Ascii(ref v) => {
		if v == "2" {
			"2-dimension"
		} else if v == "3" {
			"3-dimension"
		} else {
			return format!("Unknown ({})", v)
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

/// Interprets an Undefined tag as ASCII, when the contents are guaranteed
/// by EXIF standard to be ASCII-compatible. This function accepts UTF-8
/// strings, should they be accepted by EXIF standard in the future.
pub fn undefined_as_ascii(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Undefined(ref v, _) => {
		String::from_utf8_lossy(&v[..])
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

/// Outputs an Undefined tag as an array of bytes. Appropriate for tags
/// that are opaque and small-sized
pub fn undefined_as_u8(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Undefined(ref v, _) => {
		numarray_to_string(v)
	},
	_ => panic!(INV),
	};

	return s.to_string();
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

	match e {
	&TagValue::Undefined(ref v, le) => {
		if v.len() < 8 {
			format!("String w/ truncated preamble {}", numarray_to_string(v))
		} else if v[0..8] == ASC[..] {
			let v8 = &v[8..];
			let s = String::from_utf8_lossy(v8);
			s.into_owned()
		} else if v[0..8] == JIS[..] {
			let v8: Vec<u8> = v[8..].iter().map(|&x| x).collect();
			format!("JIS string {}", numarray_to_string(&v8))
		} else if v[0..8] == UNICODE[..] {
			let v8 = &v[8..];
			// reinterpret as vector of u16
			let v16_size = (v8.len() / 2) as u32;
			let v16 = read_u16_array(le, v16_size, v8);
			String::from_utf16_lossy(&v16)
		} else {
			format!("String w/ undefined encoding {}", numarray_to_string(v))
		}
	},
	_ => panic!(INV),
	}
}

/// Prints an opaque and long Undefined tag simply as as "blob", noting its length
pub fn undefined_as_blob(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Undefined(ref v, _) => {
		format!("Blob of {} bytes", v.len())
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn apex_tv(e: &TagValue) -> String
{
	match e {
		&TagValue::IRational(ref v) => {
			format!("{:.1} Tv APEX", v[0].value())
		},
		_ => panic!(INV),
	}
}

pub fn apex_av(e: &TagValue) -> String
{
	match e {
		&TagValue::URational(ref v) => {
			format!("{:.1} Av APEX", v[0].value())
		},
		_ => panic!(INV),
	}
}

pub fn apex_brightness(e: &TagValue) -> String
{
	match e {
		&TagValue::IRational(ref v) => {
			// numerator 0xffffffff = unknown
			if v[0].numerator == -1 {
				"Unknown".to_string()
			} else {
				format!("{:.1} APEX", v[0].value())
			}
		},
		_ => panic!(INV),
	}
}

pub fn apex_ev(e: &TagValue) -> String
{
	match e {
		&TagValue::IRational(ref v) => {
			format!("{:.2} EV APEX", v[0].value())
		},
		_ => panic!(INV),
	}
}

pub fn file_source(e: &TagValue) -> String
{
	let s = match e {
	&TagValue::Undefined(ref v, _) => {
		if v.len() > 0 && v[0] == 3 {
			"DSC"
		} else {
			"Unknown"
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn flash_energy(e: &TagValue) -> String
{
	match e {
		&TagValue::URational(ref v) => {
			format!("{} BCPS", v[0].value())
		},
		_ => panic!(INV),
	}
}

pub fn metering_mode(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Unknown",
				1 => "Average",
				2 => "Center-weighted average",
				3 => "Spot",
				4 => "Multi-spot",
				5 => "Pattern",
				6 => "Partial",
				255 => "Other",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn light_source(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
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
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn color_space(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "sRGB",
				65535 => "Uncalibrated",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn flash(e: &TagValue) -> String
{
	match e {
		&TagValue::U16(ref v) => {
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
		},
		_ => panic!(INV),
	}
}

pub fn subject_area(e: &TagValue) -> String
{
	match e {
		&TagValue::U16(ref v) => {
			match v.len() {
			2 => format!("at pixel {},{}", v[0], v[1]),
			3 => format!("at center {},{} radius {}", v[0], v[1], v[2]),
			4 => format!("at rectangle {},{} width {} height {}", v[0], v[1], v[2], v[3]),
			_ => format!("Unknown ({}) ", numarray_to_string(v)),
			}
		},
		_ => panic!(INV),
	}
}

pub fn subject_location(e: &TagValue) -> String
{
	match e {
		&TagValue::U16(ref v) => {
			format!("at pixel {},{}", v[0], v[1])
		},
		_ => panic!(INV),
	}
}

pub fn sharpness(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Normal",
				1 => "Soft",
				2 => "Hard",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn saturation(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Normal",
				1 => "Low",
				2 => "High",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn contrast(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Normal",
				1 => "Soft",
				2 => "Hard",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gain_control(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "None",
				1 => "Low gain up",
				2 => "High gain up",
				3 => "Low gain down",
				4 => "High gain down",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn exposure_mode(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Auto exposure",
				1 => "Manual exposure",
				2 => "Auto bracket",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn scene_capture_type(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Standard",
				1 => "Landscape",
				2 => "Portrait",
				3 => "Night scene",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn scene_type(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::Undefined(ref v, _) => {
			let n = v[0];
			match n {
				1 => "Directly photographed image",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn white_balance_mode(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Auto",
				1 => "Manual",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn sensing_method(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Not defined",
				2 => "One-chip color area sensor",
				3 => "Two-chip color area sensor",
				4 => "Three-chip color area sensor",
				5 => "Color sequential area sensor",
				7 => "Trilinear sensor",
				8 => "Color sequential linear sensor",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn custom_rendered(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Normal",
				1 => "Custom",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn subject_distance_range(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				0 => "Unknown",
				1 => "Macro",
				2 => "Close view",
				3 => "Distant view",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn lens_spec(e: &TagValue) -> String
{
	match e {
	&TagValue::URational(ref v) => {
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
	},

	_ => panic!(INV),

	}
}


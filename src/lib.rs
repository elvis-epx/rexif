use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
use std::fmt::Display;
use std::cell::RefCell;

mod lowlevel;
use self::lowlevel::*;
mod rational;
pub use self::rational::*;
mod types;
pub use self::types::*;
mod types_impl;
pub use self::types_impl::*;
mod debug;
use self::debug::*;

/* Detect the type of an image contained in a byte buffer */
pub fn detect_type(contents: &Vec<u8>) -> &str
{
	if contents.len() < 11 {
		return "";
	}

	if contents[0] == 0xff && contents[1] == 0xd8 &&
			contents[2] == 0xff && // contents[3] == 0xe0 &&
			contents[6] == ('J' as u8) && contents[7] == ('F' as u8) &&
			contents[8] == ('I' as u8) && contents[9] == ('F' as u8) &&
			contents[10] == 0 {
		return "image/jpeg";
	}
	if contents[0] == ('I' as u8) && contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		return "image/tiff";
	}
	if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
		return "image/tiff";
	}

	return "";
}

/* Find the embedded TIFF in a JPEG image, that contains in turn the EXIF data */
pub fn find_embedded_tiff_in_jpeg(contents: &Vec<u8>) -> (usize, usize, String)
{
	let mut err = "Scan past EOF and no EXIF found".to_string();
	
	{
	let mut offset = 2 as usize;
	let mut size: usize;

	while offset < contents.len() {
		if contents.len() < (offset + 4) {
			err = "JPEG truncated in marker header".to_string();
			break;
		}

		let marker: u16 = (contents[offset] as u16) * 256 + (contents[offset + 1] as u16);

		if marker < 0xff00 {
			err = format!("Invalid marker {:x}", marker);
			break;
		}

		offset += 2;
		size = (contents[offset] as usize) * 256 + (contents[offset + 1] as usize);

		if size < 2 {
			err = "JPEG marker size must be at least 2 (because of the size word)".to_string();
			break;
		}
		if contents.len() < (offset + size) {
			err = "JPEG truncated in marker body".to_string();
			break;
		}

		if marker == 0xffe1 {
			// Discard the size word
			offset += 2;
			size -= 2;

			if size < 6 {
				err = "EXIF preamble truncated".to_string();
				break;
			}

			if contents[offset + 0] != ('E' as u8) &&
					contents[offset + 1] != ('x' as u8) &&
					contents[offset + 2] != ('i' as u8) &&
					contents[offset + 3] != ('f' as u8) &&
					contents[offset + 4] != 0 &&
					contents[offset + 5] != 0 {
				err = "EXIF preamble unrecognized".to_string();
				break;
			}

			// Discard the 'Exif\0\0' preamble
			offset += 6;
			size -= 6;

			return (offset, size, "".to_string());
		}
		if marker == 0xffda {
			// last marker
			err = "Last mark found and no EXIF".to_string();
			break;
		}

		offset += size;
	}
	}

	return (0, 0, err);
}


fn numarray_to_string<T: Display>(numbers: &Vec<T>) -> String
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

fn tag_value(f: &IfdEntry) -> (TagValue, String)
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

/// No-op for readable value tag function
fn nop(_: &TagValue) -> String
{
	return "".to_string();
}

/// No-op for readable value tag function that should be kept as simple strings
fn strpass(_: &TagValue) -> String
{
	return "".to_string();
}

fn orientation(e: &TagValue) -> String
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

fn rational_value(e: &TagValue) -> String
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

fn rational_values(e: &TagValue) -> String
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

fn resolution_unit(e: &TagValue) -> String
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

fn exposure_time(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{} s", v[0])
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

fn f_number(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("f/{:1}", v[0].value())
		},
		_ => panic!("Invalid"),
	};

	return s.to_string();
}

/// Find a tag of given type
fn other_tag(tag: ExifTag, entries: &Vec<ExifEntry>) -> Option<&ExifEntry>
{
	for entry in entries {
		if entry.tag == tag {
			return Some(entry);
		}
	}
	None
}

/// Does postprocessing in tags that depend on other tags to be completed
fn exif_postprocessing(entry: &mut ExifEntry, entries: &Vec<ExifEntry>)
{
	match entry.tag {

	ExifTag::XResolution =>
	match other_tag(ExifTag::ResolutionUnit, entries) {
		Some(f) => {
			entry.unit = f.value_more_readable.clone();
			entry.value_more_readable.push_str(" pixels per ");
			entry.value_more_readable.push_str(&f.value_more_readable);
			},
		None => (),
	},

	ExifTag::YResolution =>
	match other_tag(ExifTag::ResolutionUnit, entries) {
		Some(f) => {
			entry.unit = f.value_more_readable.clone();
			entry.value_more_readable.push_str(" pixels per ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	_ => (),
	}
}

// FIXME check how Undefined could be converted safely to string in some cases

/* Convert a numeric tag into EXIF tag and yiels info about the tag */
fn tag_to_exif(f: u16) -> (ExifTag, &'static str, &'static str, IfdFormat, i32, i32, fn(&TagValue) -> String)
{
	match f {

	0x010e =>
	(ExifTag::ImageDescription, "Image Description", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x010f =>
	(ExifTag::Make, "Manufacturer", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x0110 =>
	(ExifTag::Model, "Model", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x0112 =>
	(ExifTag::Orientation, "Orientation", "none", IfdFormat::U16, 1, 1, orientation),

	0x011a =>
	(ExifTag::XResolution, "X Resolution", "pixels per res unit",
	IfdFormat::URational, 1, 1, rational_value),

	0x011b =>
	(ExifTag::YResolution, "Y Resolution", "pixels per res unit",
	IfdFormat::URational, 1, 1, rational_value),

	0x0128 =>
	(ExifTag::ResolutionUnit, "Resolution Unit", "none", IfdFormat::U16, 1, 1, resolution_unit),

	0x0131 =>
	(ExifTag::Software, "Software", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x0132 =>
	(ExifTag::DateTime, "Image date", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x013e =>
	(ExifTag::WhitePoint, "White Point", "CIE 1931 coordinates",
	IfdFormat::URational, 2, 2, rational_values),

	0x013f =>
	(ExifTag::PrimaryChromaticities, "Primary Chromaticities", "CIE 1931 coordinates",
	IfdFormat::URational, 6, 6, rational_values),

	0x0211 =>
	(ExifTag::YCbCrCoefficients, "YCbCr Coefficients", "none",
	IfdFormat::URational, 3, 3, rational_values),

	0x0214 =>
	(ExifTag::ReferenceBlackWhite, "Reference Black/White", "RGB or YCbCr",
	IfdFormat::URational, 6, 6, rational_values),

	0x8298 =>
	(ExifTag::Copyright, "Copyright", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x8769 =>
	(ExifTag::ExifOffset, "This image has an Exif SubIFD", "byte offset",
	IfdFormat::U32, 1, 1, nop),

	0x8825 =>
	(ExifTag::GPSOffset, "This image has a GPS SubIFD", "byte offset",
	IfdFormat::U32, 1, 1, nop),

	0x829a => (ExifTag::ExposureTime, "Exposure time", "s",
	IfdFormat::URational, 1, 1, exposure_time),

	0x829d => (ExifTag::FNumber, "Aperture", "f-number",
	IfdFormat::URational, 1, 1, f_number),

	// EPX

	// FIXME '1' means manual control, '2' program normal, '3' aperture priority, '4' shutter priority, '5' program creative (slow program), '6' program action(high-speed program), '7' portrait mode, '8' landscape mode.
	0x8822 => (ExifTag::ExposureProgram, "Exposure program", "none", IfdFormat::U16, 1, 1, nop),

	0x8824 => (ExifTag::SpectralSensitivity, "Spectral sensitivity", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME 
	0x8827 => (ExifTag::ISOSpeedRatings, "ISO speed ratings", "ISO", IfdFormat::U16, 1, 2, nop),

	0x8828 => (ExifTag::OECF, "OECF", "none", IfdFormat::Undefined, -1i32, -1i32, nop),

	0x9000 => (ExifTag::ExifVersion, "Exif version", "none", IfdFormat::Undefined, -1i32, -1i32, nop),

	0x9003 => (ExifTag::DateTimeOriginal, "Date of original image", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x9004 => (ExifTag::DateTimeDigitized, "Date of image digitalization", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0x9101 => (ExifTag::ComponentsConfiguration, "Components configuration", "", IfdFormat::Undefined, 4, 4, nop),

	0x9102 => (ExifTag::CompressedBitsPerPixel, "Compressed bits per pixel", "", IfdFormat::URational, 1, 1, nop),

	// FIXME APEX? Shutter speed. To convert this value to ordinary 'Shutter Speed'; calculate this value's power of 2, then reciprocal. For example, if value is '4', shutter speed is 1/(2^4)=1/16 second.
	0x9201 => (ExifTag::ShutterSpeedValue, "Shutter speed", "APEX", IfdFormat::IRational, 1, 1, nop),
	
	// FIXME Numerator FFFFFFFF = Unknown, The actual aperture value of lens when the image was taken. To convert this value to ordinary F-number(F-stop), calculate this value's power of root 2 (=1.4142). For example, if value is '5', F-number is 1.4142^5 = F5.6.
	0x9202 => (ExifTag::ApertureValue, "Aperture value", "APEX", IfdFormat::URational, 1, 1, nop),

	// FIXME numerator FFFF.. = Unknown
	0x9203 => (ExifTag::BrightnessValue, "Brightness value", "APEX", IfdFormat::IRational, 1, 1, nop),

	0x9204 => (ExifTag::ExposureBiasValue, "Exposure bias value", "APEX", IfdFormat::IRational, 1, 1, nop),

	0x9205 => (ExifTag::MaxApertureValue, "Maximum aperture value", "APEX", IfdFormat::URational, 1, 1, nop),

	0x9206 => (ExifTag::SubjectDistance, "Subject distance", "m", IfdFormat::URational, 1, 1, nop),

	// FIXME  '1' means average, '2' center weighted average, '3' spot, '4' multi-spot, '5' multi-segment. 6=partial, 255= other
	0x9207 => (ExifTag::MeteringMode, "Meteting mode", "none", IfdFormat::U16, 1, 1, nop),

	// FIXME http://www.awaresystems.be/imaging/tiff/tifftags/privateifd/exif/lightsource.html
	0x9208 => (ExifTag::LightSource, "Light source", "", IfdFormat::U16, 1, 1, nop),

	// FIXME
	0x9209 => (ExifTag::Flash, "Flash", "", IfdFormat::U16, 1, 2, nop),

	0x920a => (ExifTag::FocalLength, "Focal length", "mm", IfdFormat::URational, 1, 1, nop),

	// FIXME 
	0x9214 => (ExifTag::SubjectArea, "Subject area", "", IfdFormat::U16, 2, 4, nop),

	0x927c => (ExifTag::MakerNote, "Maker note", "none", IfdFormat::Undefined, -1i32, -1i32, nop),

	0x9286 => (ExifTag::UserComment, "User comment", "none", IfdFormat::Undefined, -1i32, -1i32, nop),

	0xa000 => (ExifTag::FlashPixVersion, "Flashpix version", "", IfdFormat::Undefined, -1i32, -1i32, nop),

	// FIXME
	0xa001 => (ExifTag::ColorSpace, "Color space", "", IfdFormat::U16, 1, 1, nop),

	0xa004 => (ExifTag::RelatedSoundFile, "Related sound file", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	0xa20b => (ExifTag::FlashEnergy, "Flash energy", "beam candle power seconds", IfdFormat::URational, 1, 1, nop),

	// FIXME relate to focal place resolution unit
	0xa20e => (ExifTag::FocalPlaneXResolution, "Focal plane X resolution", "", IfdFormat::URational, 1, 1, nop),

	// FIXME relate to focal place resolution unit
	0xa20f => (ExifTag::FocalPlaneYResolution, "Focal plane Y resolution", "", IfdFormat::URational, 1, 1, nop),

	// FIXME , default = in
	0xa210 => (ExifTag::FocalPlaneResolutionUnit, "Focal plane resolution unit", "", IfdFormat::U16, 1, 1, nop),

	// FIXME
	0xa214 => (ExifTag::SubjectLocation, "Subject location", "X/Y", IfdFormat::U16, 2, 2, nop),

	0xa215 => (ExifTag::ExposureIndex, "Exposure index", "FIXME", IfdFormat::URational, 1, 1, nop),

	// FIXME
	0xa217 => (ExifTag::SensingMethod, "Sensing method", "", IfdFormat::U16, 1, 1, nop),

	// FIXME
	0xa300 => (ExifTag::FileSource, "File source", "none", IfdFormat::Undefined, 1, 1, nop),

	// FIXME
	0xa301 => (ExifTag::SceneType, "Scene type", "", IfdFormat::Undefined, 1, 1, nop),

	// FIXME
	0xa302 => (ExifTag::CFAPattern, "CFA Pattern", "", IfdFormat::Undefined, -1i32, -1i32, nop),

	// FIXME
	0xa401 => (ExifTag::CustomRendered, "Custom rendered", "", IfdFormat::U16, 1, 1, nop),

	0xa402 => (ExifTag::ExposureMode,
		 "Exposure mode", "", IfdFormat::U16, 1, 1, nop),

	0xa403 => (ExifTag::WhiteBalanceMode,
		 "White balance mode", "", IfdFormat::U16, 1, 1, nop),

	0xa404 => (ExifTag::DigitalZoomRatio,
		 "Digital zoom ratio", "", IfdFormat::URational, 1, 1, nop),

	0xa405 => (ExifTag::FocalLengthIn35mmFilm,
		 "Equivalent focal length in 35mm", "mm", IfdFormat::U16, 1, 1, nop),

	0xa406 => (ExifTag::SceneCaptureType,
		 "Scene capture type", "", IfdFormat::U16, 1, 1, nop),

	0xa407 => (ExifTag::GainControl,
		 "Gain control", "", IfdFormat::U16, 1, 1, nop),

	0xa408 => (ExifTag::Contrast,
		 "Contrast", "", IfdFormat::U16, 1, 1, nop),

	0xa409 => (ExifTag::Saturation,
		 "Saturation", "", IfdFormat::U16, 1, 1, nop),

	0xa40a => (ExifTag::Sharpness,
		 "Sharpness", "", IfdFormat::U16, 1, 1, nop),

	0xa40b => (ExifTag::DeviceSettingDescription,
		 "Device setting description", "", IfdFormat::Undefined, -1i32, -1i32, nop),

	// FIXME
	0xa40c => (ExifTag::SubjectDistanceRange,
		 "Subject distance range", "", IfdFormat::U16, 1, 1, nop),

	0xa420 => (ExifTag::ImageUniqueID, "Image unique ID", "", IfdFormat::Str, -1i32, -1i32, strpass),
		
	0x0 => (ExifTag::GPSVersionID,
		 "GPS version ID", "", IfdFormat::U8, 4, 4, nop),

	// FIXME interpret
	0x1 => (ExifTag::GPSLatitudeRef,
		 "GPS latitude ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME and join with 0x1
	0x2 => (ExifTag::GPSLatitude,
		 "GPS latitude", "latitude deg.", IfdFormat::URational, 3, 3, nop),

	// FIXME interpret
	0x3 => (ExifTag::GPSLongitudeRef,
		 "GPS longitude ref", "longitude deg.", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME and join with 0x3
	0x4 => (ExifTag::GPSLongitude,
		 "GPS longitude", "degrees", IfdFormat::URational, 3, 3, nop),

	// FIXME
	0x5 => (ExifTag::GPSAltitudeRef,
		 "GPS altitude ref", "", IfdFormat::U8, 1, 1, nop),

	// FIXME
	0x6 => (ExifTag::GPSAltitude,
		 "GPS altitude", "m", IfdFormat::URational, 1, 1, nop),

	// FIXME
	0x7 => (ExifTag::GPSTimeStamp,
		 "GPS timestamp", "UTC", IfdFormat::URational, 3, 3, nop),

	0x8 => (ExifTag::GPSSatellites, "GPS satellites", "", IfdFormat::Str, -1i32, -1i32, strpass),

	// FIXME interpret
	0x9 => (ExifTag::GPSStatus,
		 "GPS status", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME interpret
	0xa => (ExifTag::GPSMeasureMode,
		 "GPS measure mode", "", IfdFormat::Str, -1i32, -1i32, nop),

	0xb => (ExifTag::GPSDOP,
		 "GPS Data Degree of Precision (DOP)", "deg.", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0xc => (ExifTag::GPSSpeedRef,
		 "GPS speed ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME join with 0xc, show value
	0xd => (ExifTag::GPSSpeed,
		 "GPS speed", "", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0xe => (ExifTag::GPSTrackRef,
		 "GPS track ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	0xf => (ExifTag::GPSTrack,
		 "GPS track", "deg.", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0x10 => (ExifTag::GPSImgDirectionRef,
		 "GPS image direction ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	0x11 => (ExifTag::GPSImgDirection,
		 "GPS image direction", "", IfdFormat::URational, 1, 1, nop),

	0x12 => (ExifTag::GPSMapDatum, "GPS map datum", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME interpret
	0x13 => (ExifTag::GPSDestLatitudeRef,
		 "GPS destination latitude ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME
	0x14 => (ExifTag::GPSDestLatitude,
		 "GPS destination latitude", "", IfdFormat::URational, 3, 3, nop),

	// FIXME interpret
	0x15 => (ExifTag::GPSDestLongitudeRef,
		 "GPS destination longitude ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME
	0x16 => (ExifTag::GPSDestLongitude,
		 "GPS destination longitude", "", IfdFormat::URational, 3, 3, nop),

	// FIXME interpret
	0x17 => (ExifTag::GPSDestBearingRef,
		 "GPS destination bearing ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME
	0x18 => (ExifTag::GPSDestBearing,
		 "GPS destination bearing", "", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0x19 => (ExifTag::GPSDestDistanceRef,
		 "GPS destination distance ref", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME
	0x1a => (ExifTag::GPSDestDistance,
		 "GPS destination distance", "", IfdFormat::URational, 1, 1, nop),

	// FIXME
	0x1b => (ExifTag::GPSProcessingMethod,
		 "GPS processing method", "", IfdFormat::Undefined, -1i32, -1i32, nop),

	// FIXME
	0x1c => (ExifTag::GPSAreaInformation,
		 "GPS area information", "", IfdFormat::Undefined, -1i32, -1i32, nop),

	0x1d => (ExifTag::GPSDateStamp, "GPS date stamp", "none", IfdFormat::Str, -1i32, -1i32, strpass),

	// FIXME
	0x1e => (ExifTag::GPSDifferential,
		 "GPS differential", "", IfdFormat::U16, 1, 1, nop),
	_ =>
	(ExifTag::UnknownToMe, "Unknown to this library, or manufacturer-specific", "Unknown unit",
		IfdFormat::Unknown, -1i32, -1i32, nop)
	}
}

/* Parse of raw IFD entry into EXIF data, if it is of a known type */
fn parse_exif_entry(f: &IfdEntry) -> ExifEntry
{
	let (value, readable_value) = tag_value(f);

	let mut e = ExifEntry {
			ifd: f.clone(),
			tag: ExifTag::UnknownToMe,
			value: value,
			unit: "Unknown".to_string(),
			tag_readable: format!("Unparsed tag {:x}", f.tag).to_string(),
			value_readable: readable_value.clone(),
			value_more_readable: readable_value,
			};

	let (tag, tag_readable, unit, format, min_count, max_count, more_readable) = tag_to_exif(f.tag);

	if tag == ExifTag::UnknownToMe {
		// Unknown EXIF tag type
		return e;
	}

	if (tag as u16) != f.tag ||
		(min_count == -1 && (format != IfdFormat::Str &&
				format != IfdFormat::Undefined &&
				format != IfdFormat::Unknown)) {
		panic!("Internal error {:x}", f.tag);
	}

	if format != f.format {
		warning(&format!("EXIF tag {:x} {}, expected format {}, found {}",
			f.tag, tag_readable, format as u8, f.format as u8));
		return e;
	}

	if min_count != -1 &&
			((f.count as i32) < min_count ||
			(f.count as i32) > max_count) {
		warning(&format!("EXIF tag {:x} {}, format {}, expected count {}..{} found {}",
			f.tag, tag_readable, format as u8, min_count,
			max_count, f.count));
		return e;
	}

	e.tag = tag;
	e.tag_readable = tag_readable.to_string();
	e.unit = unit.to_string();

	if (more_readable as *const fn(&ExifTag) -> String) != (nop as *const fn(&ExifTag) -> String) &&
			(more_readable as *const fn(&ExifTag) -> String) != (strpass as *const fn(&ExifTag) -> String) {
		e.value_more_readable = more_readable(&e.value);
	}

	return e;
}


/* Superficial parse of IFD that can't fail */
fn parse_ifd(subifd: bool, le: bool, count: u16, contents: &[u8]) -> (Vec<IfdEntry>, usize)
{
	let mut entries: Vec<IfdEntry> = Vec::new();

	for i in 0..count {
		// println!("Parsing IFD entry {}", i);
		let mut offset = (i as usize) * 12;
		let tag = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let format = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let count = read_u32(le, &contents[offset..offset + 4]);
		offset += 4;
		let data = &contents[offset..offset + 4];
		let data = data.to_vec();

		let entry = IfdEntry{tag: tag, format: to_ifdformat(format), count: count,
					ifd_data: data, le: le,
					ext_data: Vec::new(), data: Vec::new()};
		entries.push(entry);
	}

	let next_ifd = match subifd {
		true => 0,
		false => read_u32(le, &contents[count as usize * 12..]) as usize
	};

	return (entries, next_ifd);
}

/* Deep parse of IFD that grabs EXIF data from IFD0 or SubIFD */
fn parse_exif_ifd(le: bool, contents: &[u8], ioffset: usize,
				exif_entries: &mut Vec<ExifEntry>) -> InExifResult
{
	let mut offset = ioffset;

	// println!("Offset is {}", offset);
	if contents.len() < (offset + 2) {
		return Err(ExifError{
			kind: ExifErrorKind::ExifIfdTruncated,
			extra: "Truncated at dir entry count".to_string()});
	}

	let count = read_u16(le, &contents[offset..offset + 2]);
	// println!("IFD entry count is {}", count);
	let ifd_length = (count as usize) * 12;
	offset += 2;

	if contents.len() < (offset + ifd_length) {
		return Err(ExifError{
			kind: ExifErrorKind::ExifIfdTruncated,
			extra: "Truncated at dir listing".to_string()});
	}

	let (mut ifd, _) = parse_ifd(true, le, count, &contents[offset..offset + ifd_length]);

	for entry in &mut ifd {
		entry.copy_data(&contents);
		let exif_entry = parse_exif_entry(&entry);
		exif_entries.push(exif_entry);
	}

	return Ok(());
}

/* Parses IFD0 and looks for SubIFD within IFD0 */
fn parse_ifds(le: bool, ifd0_offset: usize, contents: &[u8]) -> ExifResult
{
	let mut offset = ifd0_offset;
	let mut exif_entries: Vec<ExifEntry> = Vec::new();

	// fills exif_entries with data from IFD0

	match parse_exif_ifd(le, &contents, offset, &mut exif_entries) {
		Ok(_) => true,
		Err(e) => return Err(e),
	};

	// at this point we knot that IFD0 is good
	// looks for SubIFD (EXIF)

	let count = read_u16(le, &contents[offset..offset + 2]);
	let ifd_length = (count as usize) * 12 + 4;
	offset += 2;

	let (ifd, _) = parse_ifd(false, le, count, &contents[offset..offset + ifd_length]);

	for entry in &ifd {
		if entry.tag != (ExifTag::ExifOffset as u16) &&
				entry.tag != (ExifTag::GPSOffset as u16) {
			continue;
		}

		let exif_offset = entry.data_as_offset();

		if contents.len() < exif_offset {
			return Err(ExifError{
				kind: ExifErrorKind::ExifIfdTruncated,
				extra: "Exif SubIFD goes past EOF".to_string()});
		}

		match parse_exif_ifd(le, &contents, exif_offset, &mut exif_entries) {
			Ok(_) => true,
			Err(e) => return Err(e),
		};
	}

	// I didn't want to make the copy, but how to pass a vector that is
	// being iterated onto?
	let exif_entries_copy = exif_entries.clone();

	for entry in &mut exif_entries {
		exif_postprocessing(entry, &exif_entries_copy);
	}

	return Ok(RefCell::new(ExifData{file: "".to_string(),
				size: 0,
				mime: "".to_string(),
				entries: exif_entries}));
}

/* Parse a TIFF image, or embedded TIFF in JPEG, in order to get IFDs and then the EXIF data */
pub fn parse_tiff(contents: &[u8]) -> ExifResult
{
	let mut le = false;

	if contents.len() < 8 {
		return Err(ExifError{
			kind: ExifErrorKind::TiffTruncated,
			extra: "".to_string()});
	} else if contents[0] == ('I' as u8) &&
			contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		le = true;
	} else if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
	} else {
		let err = format!("Preamble is {:x} {:x} {:x} {:x}",
			contents[0], contents[1],
			contents[2], contents[3]);
		return Err(ExifError{
			kind: ExifErrorKind::TiffBadPreamble,
			extra: err.to_string()});
	}

	let offset = read_u32(le, &contents[4..8]) as usize;

	return parse_ifds(le, offset, &contents);
}

/* Parse an image buffer that may be of any format. Detect format and find EXIF data */
pub fn parse_buffer(fname: &str, contents: &Vec<u8>) -> ExifResult
{
	let mime = detect_type(&contents);

	if mime == "" {
		return Err(ExifError{
				kind: ExifErrorKind::FileTypeUnknown,
				extra: "".to_string()});
	}

	let mut offset = 0 as usize;
	let mut size = contents.len() as usize;

	if mime == "image/jpeg" {
		let (eoffset, esize, err) = find_embedded_tiff_in_jpeg(&contents);
		if eoffset == 0 {
			return Err(ExifError{
				kind: ExifErrorKind::JpegWithoutExif,
				extra: err.to_string()});
		}
		offset = eoffset;
		size = esize;
		// println!("Offset {} size {}", offset, size);
	}
	match parse_tiff(&contents[offset .. offset + size]) {
		Ok(d) => {
				d.borrow_mut().size = contents.len();
				d.borrow_mut().file = fname.to_string();
				d.borrow_mut().mime = mime.to_string();
				Ok(d)
			},
		Err(e) => Err(e)
	}

}

/* Read and interpret an image file */
pub fn read_file(fname: &str, f: &mut File) -> ExifResult
{
	match f.seek(SeekFrom::Start(0)) {
		Ok(_) => (),
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileSeekError,
				extra: fname.to_string()}),
	}

	// TODO: should read only the relevant parts of a file,
	// and pass a StringIO-like object instead of a Vec buffer

	let mut contents: Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Ok(_) => parse_buffer(&fname, &contents),
		Err(_) => Err(ExifError{kind: ExifErrorKind::FileReadError,
				extra: fname.to_string()}),
	}
}

/* Parse an image file */
pub fn parse_file(fname: &str) -> ExifResult
{
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileOpenError,
				extra: fname.to_string()}),
	};
	return read_file(fname, &mut f);
}

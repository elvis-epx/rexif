use super::types::*;

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

/// Does postprocessing in tags that depend on other tags to have a complete interpretation
/// e.g. when the unit of a tag is annotated on another tag
pub fn exif_postprocessing(entry: &mut ExifEntry, entries: &Vec<ExifEntry>)
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

	ExifTag::FocalPlaneXResolution =>
	match other_tag(ExifTag::FocalPlaneResolutionUnit, entries) {
		Some(f) => {
			entry.unit = f.value_more_readable.clone();
			entry.value_more_readable.push_str(" pixels per ");
			entry.value_more_readable.push_str(&f.value_more_readable);
			},
		None => (),
	},

	ExifTag::FocalPlaneYResolution =>
	match other_tag(ExifTag::FocalPlaneResolutionUnit, entries) {
		Some(f) => {
			entry.unit = f.value_more_readable.clone();
			entry.value_more_readable.push_str(" pixels per ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	ExifTag::GPSLatitude =>
	match other_tag(ExifTag::GPSLatitudeRef, entries) {
		Some(f) => {
			entry.value_more_readable.push_str(" ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	ExifTag::GPSLongitude =>
	match other_tag(ExifTag::GPSLongitudeRef, entries) {
		Some(f) => {
			entry.value_more_readable.push_str(" ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	ExifTag::GPSAltitude =>
	match other_tag(ExifTag::GPSAltitudeRef, entries) {
		Some(f) => {
			let altref = match f.value {
				TagValue::U8(ref fv) => fv[0],
				_ => return ()
			};

			if altref != 0 {
				entry.value_more_readable.push_str(" below sea level");
			}
		},
		None => (),
	},

	ExifTag::GPSDestLatitude =>
	match other_tag(ExifTag::GPSDestLatitudeRef, entries) {
		Some(f) => {
			entry.value_more_readable.push_str(" ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	ExifTag::GPSDestLongitude =>
	match other_tag(ExifTag::GPSDestLongitudeRef, entries) {
		Some(f) => {
			entry.value_more_readable.push_str(" ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	ExifTag::GPSDestDistance =>
	match other_tag(ExifTag::GPSDestDistanceRef, entries) {
		Some(f) => {
			entry.unit = f.value_more_readable.clone();
			entry.value_more_readable.push_str(" ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},

	ExifTag::GPSSpeed =>
	match other_tag(ExifTag::GPSSpeedRef, entries) {
		Some(f) => {
			entry.unit = f.value_more_readable.clone();
			entry.value_more_readable.push_str(" ");
			entry.value_more_readable.push_str(&f.value_more_readable);
		},
		None => (),
	},
	_ => (),
	}
}

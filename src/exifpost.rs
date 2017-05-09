use super::types::*;

/// Find a tag of given type
fn other_tag(tag: ExifTag, entries: &Vec<ExifEntry>) -> Option<&ExifEntry>
{
	entries.iter().find(|entry| entry.tag == IfdTag::Exif(tag))
}

/// Does postprocessing in tags that depend on other tags to have a complete interpretation
/// e.g. when the unit of a tag is annotated on another tag
pub fn exif_postprocessing(entry: &mut ExifEntry, entries: &Vec<ExifEntry>)
{
	if let IfdTag::Exif(exif_tag) = entry.tag {
		match exif_tag {

			ExifTag::XResolution =>
			match other_tag(ExifTag::ResolutionUnit, entries) {
				Some(f) => {
					entry.value_more_readable.push_str(" pixels per ");
					entry.value_more_readable.push_str(&f.value_more_readable);
					},
				None => (),
			},

			ExifTag::YResolution =>
			match other_tag(ExifTag::ResolutionUnit, entries) {
				Some(f) => {
					entry.value_more_readable.push_str(" pixels per ");
					entry.value_more_readable.push_str(&f.value_more_readable);
				},
				None => (),
			},

			ExifTag::FocalPlaneXResolution =>
			match other_tag(ExifTag::FocalPlaneResolutionUnit, entries) {
				Some(f) => {
					entry.value_more_readable.push_str(" pixels per ");
					entry.value_more_readable.push_str(&f.value_more_readable);
					},
				None => (),
			},

			ExifTag::FocalPlaneYResolution =>
			match other_tag(ExifTag::FocalPlaneResolutionUnit, entries) {
				Some(f) => {
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
					entry.value_more_readable.push_str(" ");
					entry.value_more_readable.push_str(&f.value_more_readable);
				},
				None => (),
			},

			ExifTag::GPSSpeed =>
			match other_tag(ExifTag::GPSSpeedRef, entries) {
				Some(f) => {
					entry.value_more_readable.push_str(" ");
					entry.value_more_readable.push_str(&f.value_more_readable);
				},
				None => (),
			},

			_ => (),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn other_tag_should_return_an_entry_with_a_matching_tag_if_one_exists() {
		let vec = vec![
			ExifEntry {
				namespace: Namespace::Standard,
				tag: IfdTag::Exif(ExifTag::ImageDescription),
				value: TagValue::Ascii(String::new()),
				value_more_readable: String::new(),
			},
			ExifEntry {
				namespace: Namespace::Standard,
				tag: IfdTag::Exif(ExifTag::Make),
				value: TagValue::Ascii(String::new()),
				value_more_readable: String::new(),
			},
		];
		let other_entry = other_tag(ExifTag::Make, &vec);

		assert!(other_entry.is_some());
		assert!(other_entry.unwrap().tag.is_known());
		assert_eq!(ExifTag::Make as u16, other_entry.unwrap().tag.value());
	}

	#[test]
	fn other_tag_should_return_none_if_no_matching_tag_exists() {
		let vec: Vec<ExifEntry> = Vec::new();
		let other_entry = other_tag(ExifTag::Make, &vec);

		assert!(other_entry.is_none());
	}
}

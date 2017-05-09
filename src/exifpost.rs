use super::types::*;

/// Find a tag of given type
fn entry_for_tag(tag: ExifTag, entries: &Vec<ExifEntry>) -> Option<&ExifEntry>
{
	entries.iter().find(|entry| entry.tag == IfdTag::Exif(tag))
}

fn value_more_readable_for_tag(tag: ExifTag, entries: &Vec<ExifEntry>) -> Option<&String> {
	entry_for_tag(tag, entries).map(|entry| &entry.value_more_readable)
}

fn postprocess_entry(value_more_readable: &mut String, entries: &Vec<ExifEntry>, tag: ExifTag, join_text: &str) {
	if let Some(other_value_more_readable) = value_more_readable_for_tag(tag, entries) {
		value_more_readable.push_str(join_text);
		value_more_readable.push_str(other_value_more_readable);
	}
}

/// Does postprocessing in tags that depend on other tags to have a complete interpretation
/// e.g. when the unit of a tag is annotated on another tag
pub fn exif_postprocessing(entry: &mut ExifEntry, entries: &Vec<ExifEntry>)
{
	if let IfdTag::Exif(exif_tag) = entry.tag {
		match exif_tag {

			ExifTag::XResolution | ExifTag::YResolution =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::ResolutionUnit, " pixels per "),

			ExifTag::FocalPlaneXResolution | ExifTag::FocalPlaneYResolution =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::FocalPlaneResolutionUnit, " pixels per "),

			ExifTag::GPSLatitude =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::GPSLatitudeRef, " "),

			ExifTag::GPSLongitude =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::GPSLongitudeRef, " "),

			ExifTag::GPSAltitude =>
			if let Some(f) = entry_for_tag(ExifTag::GPSAltitudeRef, entries) {
				if let TagValue::U8(ref fv) = f.value {
					if fv[0] != 0 {
						entry.value_more_readable.push_str(" below sea level");
					}
				}
			},

			ExifTag::GPSDestLatitude =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::GPSDestLatitudeRef, " "),

			ExifTag::GPSDestLongitude =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::GPSDestLongitudeRef, " "),

			ExifTag::GPSDestDistance =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::GPSDestDistanceRef, " "),

			ExifTag::GPSSpeed =>
				postprocess_entry(&mut entry.value_more_readable,
					entries, ExifTag::GPSSpeedRef, " "),

			_ => (),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_entry(tag: ExifTag, value_more_readable: &str) -> ExifEntry {
		ExifEntry {
			namespace: Namespace::Standard,
			tag: IfdTag::Exif(tag),
			value: TagValue::Ascii(String::new()),
			value_more_readable: value_more_readable.to_string(),
		}
	}

	fn postprocess_entries(tag1: ExifTag,
						   value_more_readable1: &str,
					   	   tag2: ExifTag,
						   value_more_readable2: &str) -> ExifEntry
	{
		let entries = vec![
			create_entry(tag1, value_more_readable1),
			create_entry(tag2, value_more_readable2),
		];
		let mut entry = entries[0].clone();
		exif_postprocessing(&mut entry, &entries);

		entry
	}

	#[test]
	fn entry_for_tag_should_return_an_entry_with_a_matching_tag_if_one_exists() {
		let vec = vec![
			create_entry(ExifTag::ImageDescription, ""),
			create_entry(ExifTag::Make, ""),
		];
		let other_entry = entry_for_tag(ExifTag::Make, &vec);

		assert!(other_entry.is_some());
		assert!(other_entry.unwrap().tag.is_known());
		assert_eq!(ExifTag::Make as u16, other_entry.unwrap().tag.value());
	}

	#[test]
	fn entry_for_tag_should_return_none_if_no_matching_tag_exists() {
		let vec: Vec<ExifEntry> = Vec::new();
		let other_entry = entry_for_tag(ExifTag::Make, &vec);

		assert!(other_entry.is_none());
	}

	#[test]
	fn exif_postprocessing_should_process_x_resolution_correctly() {
		let entry = postprocess_entries(ExifTag::XResolution, "foo",
			ExifTag::ResolutionUnit, "bar");

		assert_eq!("foo pixels per bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_y_resolution_correctly() {
		let entry = postprocess_entries(ExifTag::YResolution, "foo",
			ExifTag::ResolutionUnit, "bar");

		assert_eq!("foo pixels per bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_focal_plane_x_resolution_correctly() {
		let entry = postprocess_entries(ExifTag::FocalPlaneXResolution, "foo",
			ExifTag::FocalPlaneResolutionUnit, "bar");

		assert_eq!("foo pixels per bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_focal_plane_y_resolution_correctly() {
		let entry = postprocess_entries(ExifTag::FocalPlaneYResolution, "foo",
			ExifTag::FocalPlaneResolutionUnit, "bar");

		assert_eq!("foo pixels per bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_latitude_correctly() {
		let entry = postprocess_entries(ExifTag::GPSLatitude, "foo",
			ExifTag::GPSLatitudeRef, "bar");

		assert_eq!("foo bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_longitude_correctly() {
		let entry = postprocess_entries(ExifTag::GPSLongitude, "foo",
			ExifTag::GPSLongitudeRef, "bar");

		assert_eq!("foo bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_altitude_above_sea_level_correctly() {
		let entries = vec![
			ExifEntry {
			   	namespace: Namespace::Standard,
			   	tag: IfdTag::Exif(ExifTag::GPSAltitude),
			   	value: TagValue::Ascii(String::new()),
			   	value_more_readable: "foo".to_string(),
		   },
		   ExifEntry {
			   	namespace: Namespace::Standard,
		   		tag: IfdTag::Exif(ExifTag::GPSAltitudeRef),
	   			value: TagValue::U8(vec![0]),
	   			value_more_readable: String::new(),
	   		},
		];
		let mut entry = entries[0].clone();
		exif_postprocessing(&mut entry, &entries);

		assert_eq!("foo", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_altitude_below_sea_level_correctly() {
		let entries = vec![
			ExifEntry {
			   	namespace: Namespace::Standard,
			   	tag: IfdTag::Exif(ExifTag::GPSAltitude),
			   	value: TagValue::Ascii(String::new()),
			   	value_more_readable: "foo".to_string(),
		   },
		   ExifEntry {
			   	namespace: Namespace::Standard,
		   		tag: IfdTag::Exif(ExifTag::GPSAltitudeRef),
	   			value: TagValue::U8(vec![1]),
	   			value_more_readable: String::new(),
	   		},
		];
		let mut entry = entries[0].clone();
		exif_postprocessing(&mut entry, &entries);

		assert_eq!("foo below sea level", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_dest_latitude_correctly() {
		let entry = postprocess_entries(ExifTag::GPSDestLatitude, "foo",
			ExifTag::GPSDestLatitudeRef, "bar");

		assert_eq!("foo bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_dest_longitude_correctly() {
		let entry = postprocess_entries(ExifTag::GPSDestLongitude, "foo",
			ExifTag::GPSDestLongitudeRef, "bar");

		assert_eq!("foo bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_dest_distance_correctly() {
		let entry = postprocess_entries(ExifTag::GPSDestDistance, "foo",
			ExifTag::GPSDestDistanceRef, "bar");

		assert_eq!("foo bar", entry.value_more_readable);
	}

	#[test]
	fn exif_postprocessing_should_process_gps_speed_correctly() {
		let entry = postprocess_entries(ExifTag::GPSSpeed, "foo",
			ExifTag::GPSSpeedRef, "bar");

		assert_eq!("foo bar", entry.value_more_readable);
	}
}

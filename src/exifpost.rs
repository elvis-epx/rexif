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

/// Does postprocessing in tags that depend on other tags to be completed
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

	_ => (),
	}
}

// FIXME check how Undefined could be converted safely to string in some cases

use super::types::*;
use super::exifreadable::*;

/// Convert a numeric tag into ExifTag enumeration, and yields information about the tag. This information
/// is used by the main body of the parser to sanity-check the tags found in image
/// and make sure that EXIF tags have the right data types
pub fn tag_to_exif(f: u16) -> (ExifTag, IfdFormat, i32, i32,
					           fn(&TagValue) -> String)
{
	match f {

	0x010e =>
	(ExifTag::ImageDescription, IfdFormat::Ascii,
	-1i32, -1i32, strpass),

	0x010f =>
	(ExifTag::Make, IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x013c =>
	(ExifTag::HostComputer, IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0110 =>
	(ExifTag::Model, IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0112 =>
	(ExifTag::Orientation, IfdFormat::U16, 1, 1, orientation),

	0x011a =>
	(ExifTag::XResolution,
	IfdFormat::URational, 1, 1, rational_value),

	0x011b =>
	(ExifTag::YResolution,
	IfdFormat::URational, 1, 1, rational_value),

	0x0128 =>
	(ExifTag::ResolutionUnit, IfdFormat::U16, 1, 1, resolution_unit),

	0x0131 =>
	(ExifTag::Software, IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0132 =>
	(ExifTag::DateTime, IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x013e =>
	(ExifTag::WhitePoint,
	IfdFormat::URational, 2, 2, rational_values),

	0x013f =>
	(ExifTag::PrimaryChromaticities,
	IfdFormat::URational, 6, 6, rational_values),

	0x0211 =>
	(ExifTag::YCbCrCoefficients,
	IfdFormat::URational, 3, 3, rational_values),

	0x0214 =>
	(ExifTag::ReferenceBlackWhite,
	IfdFormat::URational, 6, 6, rational_values),

	0x8298 =>
	(ExifTag::Copyright, IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x8769 =>
	(ExifTag::ExifOffset,
	IfdFormat::U32, 1, 1, strpass),

	0x8825 =>
	(ExifTag::GPSOffset,
	IfdFormat::U32, 1, 1, strpass),

	0x829a =>
	(ExifTag::ExposureTime,
	IfdFormat::URational, 1, 1, exposure_time),

	0x829d =>
	(ExifTag::FNumber,
	IfdFormat::URational, 1, 1, f_number),

	0x8822 =>
	(ExifTag::ExposureProgram,
	IfdFormat::U16, 1, 1, exposure_program),

	0x8824 =>
	(ExifTag::SpectralSensitivity,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x8827 =>
	(ExifTag::ISOSpeedRatings,
	IfdFormat::U16, 1, 3, iso_speeds),

	0x8828 =>
	(ExifTag::OECF,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_blob),

	0x9000 =>
	(ExifTag::ExifVersion,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_ascii),

	0x9003 =>
	(ExifTag::DateTimeOriginal,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x9004 =>
	(ExifTag::DateTimeDigitized,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x9201 =>
	(ExifTag::ShutterSpeedValue,
	IfdFormat::IRational, 1, 1, apex_tv),

	0x9202 =>
	(ExifTag::ApertureValue,
	IfdFormat::URational, 1, 1, apex_av),

	0x9203 =>
	(ExifTag::BrightnessValue,
	IfdFormat::IRational, 1, 1, apex_brightness),

	0x9204 =>
	(ExifTag::ExposureBiasValue,
	IfdFormat::IRational, 1, 1, apex_ev),

	0x9205 =>
	(ExifTag::MaxApertureValue, IfdFormat::URational, 1, 1, apex_av),

	0x9206 =>
	(ExifTag::SubjectDistance,
	IfdFormat::URational, 1, 1, meters),

	0x9207 =>
	(ExifTag::MeteringMode,
	IfdFormat::U16, 1, 1, metering_mode),

	0x9208 =>
	(ExifTag::LightSource,
	IfdFormat::U16, 1, 1, light_source),

	0x9209 => (ExifTag::Flash,
	IfdFormat::U16, 1, 2, flash),

	0x920a =>
	(ExifTag::FocalLength,
	IfdFormat::URational, 1, 1, focal_length),

	0x9214 =>
	(ExifTag::SubjectArea,
	IfdFormat::U16, 2, 4, subject_area),

	0x927c =>
	(ExifTag::MakerNote,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_blob),

	0x9286 =>
	(ExifTag::UserComment,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_encoded_string),

	0xa000 =>
	(ExifTag::FlashPixVersion,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_ascii),

	0xa001 =>
	(ExifTag::ColorSpace,
	IfdFormat::U16, 1, 1, color_space),

	0xa004 =>
	(ExifTag::RelatedSoundFile,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0xa20b => (ExifTag::FlashEnergy,
	IfdFormat::URational, 1, 1, flash_energy),

	0xa20e =>
	(ExifTag::FocalPlaneXResolution,
	IfdFormat::URational, 1, 1, rational_value),

	0xa20f =>
	(ExifTag::FocalPlaneYResolution,
	IfdFormat::URational, 1, 1, rational_value),

	0xa210 =>
	(ExifTag::FocalPlaneResolutionUnit,
	IfdFormat::U16, 1, 1, resolution_unit),

	0xa214 =>
	(ExifTag::SubjectLocation,
	IfdFormat::U16, 2, 2, subject_location),

	// TODO check if rational as decimal value is the best for this one
	0xa215 =>
	(ExifTag::ExposureIndex,
	IfdFormat::URational, 1, 1, rational_value),

	0xa217 =>
	(ExifTag::SensingMethod,
	IfdFormat::U16, 1, 1, sensing_method),

	0xa300 =>
	(ExifTag::FileSource,
	IfdFormat::Undefined, 1, 1, file_source),

	0xa301 =>
	(ExifTag::SceneType,
	IfdFormat::Undefined, 1, 1, scene_type),

	0xa302 =>
	(ExifTag::CFAPattern,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_u8),

	0xa401 =>
	(ExifTag::CustomRendered,
	IfdFormat::U16, 1, 1, custom_rendered),

	0xa402 =>
	(ExifTag::ExposureMode,
	IfdFormat::U16, 1, 1, exposure_mode),

	0xa403 =>
	(ExifTag::WhiteBalanceMode,
	IfdFormat::U16, 1, 1, white_balance_mode),

	0xa404 =>
	(ExifTag::DigitalZoomRatio,
	IfdFormat::URational, 1, 1, rational_value),

	0xa405 =>
	(ExifTag::FocalLengthIn35mmFilm,
	IfdFormat::U16, 1, 1, focal_length_35),

	0xa406 =>
	(ExifTag::SceneCaptureType,
	IfdFormat::U16, 1, 1, scene_capture_type),

	0xa407 =>
	(ExifTag::GainControl,
	IfdFormat::U16, 1, 1, gain_control),

	0xa408 =>
	(ExifTag::Contrast,
	IfdFormat::U16, 1, 1, contrast),

	0xa409 =>
	(ExifTag::Saturation,
	IfdFormat::U16, 1, 1, saturation),

	0xa40a =>
	(ExifTag::Sharpness,
	IfdFormat::U16, 1, 1, sharpness),

	0xa432 =>
	(ExifTag::LensSpecification,
	IfdFormat::URational, 4, 4, lens_spec),

	0xa433 =>
	(ExifTag::LensMake,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0xa434 =>
	(ExifTag::LensModel,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	// collaborate if you have any idea how to interpret this
	0xa40b =>
	(ExifTag::DeviceSettingDescription,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_blob),

	0xa40c =>
	(ExifTag::SubjectDistanceRange,
	IfdFormat::U16, 1, 1, subject_distance_range),

	0xa420 =>
	(ExifTag::ImageUniqueID,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0 =>
	(ExifTag::GPSVersionID,
	IfdFormat::U8, 4, 4, strpass),

	0x1 =>
	(ExifTag::GPSLatitudeRef,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x2 =>
	(ExifTag::GPSLatitude,
	IfdFormat::URational, 3, 3, dms),

	0x3 =>
	(ExifTag::GPSLongitudeRef,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x4 =>
	(ExifTag::GPSLongitude,
	IfdFormat::URational, 3, 3, dms),

	0x5 =>
	(ExifTag::GPSAltitudeRef,
	IfdFormat::U8, 1, 1, gps_alt_ref),

	0x6 =>
	(ExifTag::GPSAltitude,
	IfdFormat::URational, 1, 1, meters),

	0x7 =>
	(ExifTag::GPSTimeStamp,
	IfdFormat::URational, 3, 3, gpstimestamp),

	0x8 => (ExifTag::GPSSatellites,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x9 => (ExifTag::GPSStatus,
	IfdFormat::Ascii, -1i32, -1i32, gpsstatus),

	0xa => (ExifTag::GPSMeasureMode,
	IfdFormat::Ascii, -1i32, -1i32, gpsmeasuremode),

	0xb =>
	(ExifTag::GPSDOP,
	IfdFormat::URational, 1, 1, rational_value),

	0xc =>
	(ExifTag::GPSSpeedRef,
	IfdFormat::Ascii, -1i32, -1i32, gpsspeedref),

	0xd =>
	(ExifTag::GPSSpeed,
	IfdFormat::URational, 1, 1, gpsspeed),

	0xe =>
	(ExifTag::GPSTrackRef,
	IfdFormat::Ascii, -1i32, -1i32, gpsbearingref),

	0xf =>
	(ExifTag::GPSTrack,
	IfdFormat::URational, 1, 1, gpsbearing),

	0x10 =>
	(ExifTag::GPSImgDirectionRef,
	IfdFormat::Ascii, -1i32, -1i32, gpsbearingref),

	0x11 =>
	(ExifTag::GPSImgDirection,
	IfdFormat::URational, 1, 1, gpsbearing),

	0x12 =>
	(ExifTag::GPSMapDatum,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x13 =>
	(ExifTag::GPSDestLatitudeRef,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x14 =>
	(ExifTag::GPSDestLatitude,
	IfdFormat::URational, 3, 3, dms),

	0x15 =>
	(ExifTag::GPSDestLongitudeRef,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x16 =>
	(ExifTag::GPSDestLongitude,
	IfdFormat::URational, 3, 3, dms),

	0x17 =>
	(ExifTag::GPSDestBearingRef,
	IfdFormat::Ascii, -1i32, -1i32, gpsbearingref),

	0x18 =>
	(ExifTag::GPSDestBearing,
	IfdFormat::URational, 1, 1, gpsbearing),

	0x19 =>
	(ExifTag::GPSDestDistanceRef,
	IfdFormat::Ascii, -1i32, -1i32, gpsdestdistanceref),

	0x1a =>
	(ExifTag::GPSDestDistance,
	IfdFormat::URational, 1, 1, gpsdestdistance),

	0x1b =>
	(ExifTag::GPSProcessingMethod,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_encoded_string),

	0x1c =>
	(ExifTag::GPSAreaInformation,
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_encoded_string),

	0x1d =>
	(ExifTag::GPSDateStamp,
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x1e =>
	(ExifTag::GPSDifferential,
	IfdFormat::U16, 1, 1, gpsdiff),

	_ =>
	(ExifTag::UnknownToMe,
	IfdFormat::Unknown, -1i32, -1i32, nop)

	}
}

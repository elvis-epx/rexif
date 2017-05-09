use super::types::*;
use super::exifreadable::*;

/// First u32 is the minimum bound, second is the maximum bound.
pub type CountBounds = (u32, u32);

/// Convert a numeric tag into ExifTag enumeration, and yields information about the tag. This information
/// is used by the main body of the parser to sanity-check the tags found in image
/// and make sure that EXIF tags have the right data types
pub fn tag_to_exif(f: u16) -> (IfdTag, IfdFormat, Option<CountBounds>,
					           fn(&TagValue) -> String)
{

	match f {

	0x010e =>
    (IfdTag::Exif(ExifTag::ImageDescription), IfdFormat::Ascii,
	None, strpass),

	0x010f =>
    (IfdTag::Exif(ExifTag::Make), IfdFormat::Ascii, None, strpass),

	0x013c =>
    (IfdTag::Exif(ExifTag::HostComputer), IfdFormat::Ascii, None, strpass),

	0x0110 =>
    (IfdTag::Exif(ExifTag::Model), IfdFormat::Ascii, None, strpass),

	0x0112 =>
    (IfdTag::Exif(ExifTag::Orientation), IfdFormat::U16, Some((1, 1)), orientation),

	0x011a =>
    (IfdTag::Exif(ExifTag::XResolution),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0x011b =>
    (IfdTag::Exif(ExifTag::YResolution),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0x0128 =>
    (IfdTag::Exif(ExifTag::ResolutionUnit), IfdFormat::U16, Some((1, 1)), resolution_unit),

	0x0131 =>
    (IfdTag::Exif(ExifTag::Software), IfdFormat::Ascii, None, strpass),

	0x0132 =>
    (IfdTag::Exif(ExifTag::DateTime), IfdFormat::Ascii, None, strpass),

	0x013e =>
    (IfdTag::Exif(ExifTag::WhitePoint),
	IfdFormat::URational, Some((2, 2)), rational_values),

	0x013f =>
    (IfdTag::Exif(ExifTag::PrimaryChromaticities),
	IfdFormat::URational, Some((6, 6)), rational_values),

	0x0211 =>
    (IfdTag::Exif(ExifTag::YCbCrCoefficients),
	IfdFormat::URational, Some((3, 3)), rational_values),

	0x0214 =>
    (IfdTag::Exif(ExifTag::ReferenceBlackWhite),
	IfdFormat::URational, Some((6, 6)), rational_values),

	0x8298 =>
    (IfdTag::Exif(ExifTag::Copyright), IfdFormat::Ascii, None, strpass),

	0x8769 =>
    (IfdTag::Exif(ExifTag::ExifOffset),
	IfdFormat::U32, Some((1, 1)), strpass),

	0x8825 =>
    (IfdTag::Exif(ExifTag::GPSInfo),
	IfdFormat::U32, Some((1, 1)), strpass),

	0x829a =>
    (IfdTag::Exif(ExifTag::ExposureTime),
	IfdFormat::URational, Some((1, 1)), exposure_time),

	0x829d =>
    (IfdTag::Exif(ExifTag::FNumber),
	IfdFormat::URational, Some((1, 1)), f_number),

	0x8822 =>
    (IfdTag::Exif(ExifTag::ExposureProgram),
	IfdFormat::U16, Some((1, 1)), exposure_program),

	0x8824 =>
    (IfdTag::Exif(ExifTag::SpectralSensitivity),
	IfdFormat::Ascii, None, strpass),

	0x8827 =>
    (IfdTag::Exif(ExifTag::PhotographicSensitivity),
	IfdFormat::U16, Some((1, 3)), iso_speeds),

	0x8828 =>
    (IfdTag::Exif(ExifTag::OECF),
	IfdFormat::Undefined, None, undefined_as_blob),

	0x9000 =>
    (IfdTag::Exif(ExifTag::ExifVersion),
	IfdFormat::Undefined, None, undefined_as_ascii),

	0x9003 =>
    (IfdTag::Exif(ExifTag::DateTimeOriginal),
	IfdFormat::Ascii, None, strpass),

	0x9004 =>
    (IfdTag::Exif(ExifTag::DateTimeDigitized),
	IfdFormat::Ascii, None, strpass),

	0x9201 =>
    (IfdTag::Exif(ExifTag::ShutterSpeedValue),
	IfdFormat::IRational, Some((1, 1)), apex_tv),

	0x9202 =>
    (IfdTag::Exif(ExifTag::ApertureValue),
	IfdFormat::URational, Some((1, 1)), apex_av),

	0x9203 =>
    (IfdTag::Exif(ExifTag::BrightnessValue),
	IfdFormat::IRational, Some((1, 1)), apex_brightness),

	0x9204 =>
    (IfdTag::Exif(ExifTag::ExposureBiasValue),
	IfdFormat::IRational, Some((1, 1)), apex_ev),

	0x9205 =>
    (IfdTag::Exif(ExifTag::MaxApertureValue), IfdFormat::URational, Some((1, 1)), apex_av),

	0x9206 =>
    (IfdTag::Exif(ExifTag::SubjectDistance),
	IfdFormat::URational, Some((1, 1)), meters),

	0x9207 =>
    (IfdTag::Exif(ExifTag::MeteringMode),
	IfdFormat::U16, Some((1, 1)), metering_mode),

	0x9208 =>
    (IfdTag::Exif(ExifTag::LightSource),
	IfdFormat::U16, Some((1, 1)), light_source),

	0x9209 => (IfdTag::Exif(ExifTag::Flash),
	IfdFormat::U16, Some((1, 2)), flash),

	0x920a =>
    (IfdTag::Exif(ExifTag::FocalLength),
	IfdFormat::URational, Some((1, 1)), focal_length),

	0x9214 =>
    (IfdTag::Exif(ExifTag::SubjectArea),
	IfdFormat::U16, Some((2, 4)), subject_area),

	0x927c =>
    (IfdTag::Exif(ExifTag::MakerNote),
	IfdFormat::Undefined, None, undefined_as_blob),

	0x9286 =>
    (IfdTag::Exif(ExifTag::UserComment),
	IfdFormat::Undefined, None, undefined_as_encoded_string),

	0xa000 =>
    (IfdTag::Exif(ExifTag::FlashpixVersion),
	IfdFormat::Undefined, None, undefined_as_ascii),

	0xa001 =>
    (IfdTag::Exif(ExifTag::ColorSpace),
	IfdFormat::U16, Some((1, 1)), color_space),

	0xa004 =>
    (IfdTag::Exif(ExifTag::RelatedSoundFile),
	IfdFormat::Ascii, None, strpass),

	0xa20b => (IfdTag::Exif(ExifTag::FlashEnergy),
	IfdFormat::URational, Some((1, 1)), flash_energy),

	0xa20e =>
    (IfdTag::Exif(ExifTag::FocalPlaneXResolution),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0xa20f =>
    (IfdTag::Exif(ExifTag::FocalPlaneYResolution),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0xa210 =>
    (IfdTag::Exif(ExifTag::FocalPlaneResolutionUnit),
	IfdFormat::U16, Some((1, 1)), resolution_unit),

	0xa214 =>
    (IfdTag::Exif(ExifTag::SubjectLocation),
	IfdFormat::U16, Some((2, 2)), subject_location),

	// TODO check if rational as decimal value is the best for this one
	0xa215 =>
    (IfdTag::Exif(ExifTag::ExposureIndex),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0xa217 =>
    (IfdTag::Exif(ExifTag::SensingMethod),
	IfdFormat::U16, Some((1, 1)), sensing_method),

	0xa300 =>
    (IfdTag::Exif(ExifTag::FileSource),
	IfdFormat::Undefined, Some((1, 1)), file_source),

	0xa301 =>
    (IfdTag::Exif(ExifTag::SceneType),
	IfdFormat::Undefined, Some((1, 1)), scene_type),

	0xa302 =>
    (IfdTag::Exif(ExifTag::CFAPattern),
	IfdFormat::Undefined, None, undefined_as_u8),

	0xa401 =>
    (IfdTag::Exif(ExifTag::CustomRendered),
	IfdFormat::U16, Some((1, 1)), custom_rendered),

	0xa402 =>
    (IfdTag::Exif(ExifTag::ExposureMode),
	IfdFormat::U16, Some((1, 1)), exposure_mode),

	0xa403 =>
    (IfdTag::Exif(ExifTag::WhiteBalance),
	IfdFormat::U16, Some((1, 1)), white_balance_mode),

	0xa404 =>
    (IfdTag::Exif(ExifTag::DigitalZoomRatio),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0xa405 =>
    (IfdTag::Exif(ExifTag::FocalLengthIn35mmFilm),
	IfdFormat::U16, Some((1, 1)), focal_length_35),

	0xa406 =>
    (IfdTag::Exif(ExifTag::SceneCaptureType),
	IfdFormat::U16, Some((1, 1)), scene_capture_type),

	0xa407 =>
    (IfdTag::Exif(ExifTag::GainControl),
	IfdFormat::U16, Some((1, 1)), gain_control),

	0xa408 =>
    (IfdTag::Exif(ExifTag::Contrast),
	IfdFormat::U16, Some((1, 1)), contrast),

	0xa409 =>
    (IfdTag::Exif(ExifTag::Saturation),
	IfdFormat::U16, Some((1, 1)), saturation),

	0xa40a =>
    (IfdTag::Exif(ExifTag::Sharpness),
	IfdFormat::U16, Some((1, 1)), sharpness),

	0xa432 =>
    (IfdTag::Exif(ExifTag::LensSpecification),
	IfdFormat::URational, Some((4, 4)), lens_spec),

	0xa433 =>
    (IfdTag::Exif(ExifTag::LensMake),
	IfdFormat::Ascii, None, strpass),

	0xa434 =>
    (IfdTag::Exif(ExifTag::LensModel),
	IfdFormat::Ascii, None, strpass),

	// collaborate if you have any idea how to interpret this
	0xa40b =>
    (IfdTag::Exif(ExifTag::DeviceSettingDescription),
	IfdFormat::Undefined, None, undefined_as_blob),

	0xa40c =>
    (IfdTag::Exif(ExifTag::SubjectDistanceRange),
	IfdFormat::U16, Some((1, 1)), subject_distance_range),

	0xa420 =>
    (IfdTag::Exif(ExifTag::ImageUniqueID),
	IfdFormat::Ascii, None, strpass),

	0x0 =>
    (IfdTag::Exif(ExifTag::GPSVersionID),
	IfdFormat::U8, Some((4, 4)), strpass),

	0x1 =>
    (IfdTag::Exif(ExifTag::GPSLatitudeRef),
	IfdFormat::Ascii, None, strpass),

	0x2 =>
    (IfdTag::Exif(ExifTag::GPSLatitude),
	IfdFormat::URational, Some((3, 3)), dms),

	0x3 =>
    (IfdTag::Exif(ExifTag::GPSLongitudeRef),
	IfdFormat::Ascii, None, strpass),

	0x4 =>
    (IfdTag::Exif(ExifTag::GPSLongitude),
	IfdFormat::URational, Some((3, 3)), dms),

	0x5 =>
    (IfdTag::Exif(ExifTag::GPSAltitudeRef),
	IfdFormat::U8, Some((1, 1)), gps_alt_ref),

	0x6 =>
    (IfdTag::Exif(ExifTag::GPSAltitude),
	IfdFormat::URational, Some((1, 1)), meters),

	0x7 =>
    (IfdTag::Exif(ExifTag::GPSTimeStamp),
	IfdFormat::URational, Some((3, 3)), gpstimestamp),

	0x8 => (IfdTag::Exif(ExifTag::GPSSatellites),
	IfdFormat::Ascii, None, strpass),

	0x9 => (IfdTag::Exif(ExifTag::GPSStatus),
	IfdFormat::Ascii, None, gpsstatus),

	0xa => (IfdTag::Exif(ExifTag::GPSMeasureMode),
	IfdFormat::Ascii, None, gpsmeasuremode),

	0xb =>
    (IfdTag::Exif(ExifTag::GPSDOP),
	IfdFormat::URational, Some((1, 1)), rational_value),

	0xc =>
    (IfdTag::Exif(ExifTag::GPSSpeedRef),
	IfdFormat::Ascii, None, gpsspeedref),

	0xd =>
    (IfdTag::Exif(ExifTag::GPSSpeed),
	IfdFormat::URational, Some((1, 1)), gpsspeed),

	0xe =>
    (IfdTag::Exif(ExifTag::GPSTrackRef),
	IfdFormat::Ascii, None, gpsbearingref),

	0xf =>
    (IfdTag::Exif(ExifTag::GPSTrack),
	IfdFormat::URational, Some((1, 1)), gpsbearing),

	0x10 =>
    (IfdTag::Exif(ExifTag::GPSImgDirectionRef),
	IfdFormat::Ascii, None, gpsbearingref),

	0x11 =>
    (IfdTag::Exif(ExifTag::GPSImgDirection),
	IfdFormat::URational, Some((1, 1)), gpsbearing),

	0x12 =>
    (IfdTag::Exif(ExifTag::GPSMapDatum),
	IfdFormat::Ascii, None, strpass),

	0x13 =>
    (IfdTag::Exif(ExifTag::GPSDestLatitudeRef),
	IfdFormat::Ascii, None, strpass),

	0x14 =>
    (IfdTag::Exif(ExifTag::GPSDestLatitude),
	IfdFormat::URational, Some((3, 3)), dms),

	0x15 =>
    (IfdTag::Exif(ExifTag::GPSDestLongitudeRef),
	IfdFormat::Ascii, None, strpass),

	0x16 =>
    (IfdTag::Exif(ExifTag::GPSDestLongitude),
	IfdFormat::URational, Some((3, 3)), dms),

	0x17 =>
    (IfdTag::Exif(ExifTag::GPSDestBearingRef),
	IfdFormat::Ascii, None, gpsbearingref),

	0x18 =>
    (IfdTag::Exif(ExifTag::GPSDestBearing),
	IfdFormat::URational, Some((1, 1)), gpsbearing),

	0x19 =>
    (IfdTag::Exif(ExifTag::GPSDestDistanceRef),
	IfdFormat::Ascii, None, gpsdestdistanceref),

	0x1a =>
    (IfdTag::Exif(ExifTag::GPSDestDistance),
	IfdFormat::URational, Some((1, 1)), gpsdestdistance),

	0x1b =>
    (IfdTag::Exif(ExifTag::GPSProcessingMethod),
	IfdFormat::Undefined, None, undefined_as_encoded_string),

	0x1c =>
    (IfdTag::Exif(ExifTag::GPSAreaInformation),
	IfdFormat::Undefined, None, undefined_as_encoded_string),

	0x1d =>
    (IfdTag::Exif(ExifTag::GPSDateStamp),
	IfdFormat::Ascii, None, strpass),

	0x1e =>
    (IfdTag::Exif(ExifTag::GPSDifferential),
	IfdFormat::U16, Some((1, 1)), gpsdiff),

	tag @ _ =>
    (IfdTag::Unknown(tag),
	IfdFormat::Unknown, None, strpass)

	}
}

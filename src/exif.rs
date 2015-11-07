use super::types::*;
use super::exifreadable::*;

/// Convert a namespace + numeric tag into ExifTag enumeration, and yields information about the tag. This information
/// is used by the main body of the parser to sanity-check the tags found in image
/// and make sure that EXIF tags have the right data types
pub fn tag_to_exif(f: u32) -> (ExifTag, &'static str, &'static str, IfdFormat, i32, i32,
						fn(&TagValue, s: &String) -> String)
{
	match f {

	0x0000010e =>
	(ExifTag::ImageDescription, "Image Description", "none", IfdFormat::Ascii,
	-1i32, -1i32, strpass),

	0x0000010f =>
	(ExifTag::Make, "Manufacturer", "none", IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0000013c =>
	(ExifTag::HostComputer, "Host computer", "none", IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00000110 =>
	(ExifTag::Model, "Model", "none", IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00000112 =>
	(ExifTag::Orientation, "Orientation", "none", IfdFormat::U16, 1, 1, orientation),

	0x0000011a =>
	(ExifTag::XResolution, "X Resolution", "pixels per res unit",
	IfdFormat::URational, 1, 1, rational_value),

	0x0000011b =>
	(ExifTag::YResolution, "Y Resolution", "pixels per res unit",
	IfdFormat::URational, 1, 1, rational_value),

	0x00000128 =>
	(ExifTag::ResolutionUnit, "Resolution Unit", "none", IfdFormat::U16, 1, 1, resolution_unit),

	0x00000131 =>
	(ExifTag::Software, "Software", "none", IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00000132 =>
	(ExifTag::DateTime, "Image date", "none", IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0000013e =>
	(ExifTag::WhitePoint, "White Point", "CIE 1931 coordinates",
	IfdFormat::URational, 2, 2, rational_values),

	0x0000013f =>
	(ExifTag::PrimaryChromaticities, "Primary Chromaticities", "CIE 1931 coordinates",
	IfdFormat::URational, 6, 6, rational_values),

	0x00000211 =>
	(ExifTag::YCbCrCoefficients, "YCbCr Coefficients", "none",
	IfdFormat::URational, 3, 3, rational_values),

	0x00000214 =>
	(ExifTag::ReferenceBlackWhite, "Reference Black/White", "RGB or YCbCr",
	IfdFormat::URational, 6, 6, rational_values),

	0x00008298 =>
	(ExifTag::Copyright, "Copyright", "none", IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00008769 =>
	(ExifTag::ExifOffset, "This image has an Exif SubIFD", "byte offset",
	IfdFormat::U32, 1, 1, strpass),

	0x00008825 =>
	(ExifTag::GPSOffset, "This image has a GPS SubIFD", "byte offset",
	IfdFormat::U32, 1, 1, strpass),

	0x0000829a =>
	(ExifTag::ExposureTime, "Exposure time", "s",
	IfdFormat::URational, 1, 1, exposure_time),

	0x0000829d =>
	(ExifTag::FNumber, "Aperture", "f-number",
	IfdFormat::URational, 1, 1, f_number),

	0x00008822 =>
	(ExifTag::ExposureProgram, "Exposure program", "none",
	IfdFormat::U16, 1, 1, exposure_program),

	0x00008824 =>
	(ExifTag::SpectralSensitivity, "Spectral sensitivity", "ASTM string",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00008827 =>
	(ExifTag::ISOSpeedRatings, "ISO speed ratings", "ISO",
	IfdFormat::U16, 1, 3, iso_speeds),

	0x00008828 =>
	(ExifTag::OECF, "OECF", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_blob),

	0x00009000 =>
	(ExifTag::ExifVersion, "Exif version", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_ascii),

	0x00009003 =>
	(ExifTag::DateTimeOriginal, "Date of original image", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00009004 =>
	(ExifTag::DateTimeDigitized, "Date of image digitalization", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00009201 =>
	(ExifTag::ShutterSpeedValue, "Shutter speed", "APEX",
	IfdFormat::IRational, 1, 1, apex_tv),
	
	0x00009202 =>
	(ExifTag::ApertureValue, "Aperture value", "APEX",
	IfdFormat::URational, 1, 1, apex_av),

	0x00009203 =>
	(ExifTag::BrightnessValue, "Brightness value", "APEX",
	IfdFormat::IRational, 1, 1, apex_brightness),

	0x00009204 =>
	(ExifTag::ExposureBiasValue, "Exposure bias value", "APEX",
	IfdFormat::IRational, 1, 1, apex_ev),

	0x00009205 =>
	(ExifTag::MaxApertureValue, "Maximum aperture value",
	"APEX", IfdFormat::URational, 1, 1, apex_av),

	0x00009206 =>
	(ExifTag::SubjectDistance, "Subject distance", "m",
	IfdFormat::URational, 1, 1, meters),

	0x00009207 =>
	(ExifTag::MeteringMode, "Meteting mode", "none",
	IfdFormat::U16, 1, 1, metering_mode),

	0x00009208 =>
	(ExifTag::LightSource, "Light source", "none",
	IfdFormat::U16, 1, 1, light_source),

	0x00009209 => (ExifTag::Flash, "Flash", "none",
	IfdFormat::U16, 1, 2, flash),

	0x0000920a =>
	(ExifTag::FocalLength, "Focal length", "mm",
	IfdFormat::URational, 1, 1, focal_length),

	0x00009214 =>
	(ExifTag::SubjectArea, "Subject area", "px",
	IfdFormat::U16, 2, 4, subject_area),

	0x0000927c =>
	(ExifTag::MakerNote, "Maker note", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_blob),

	0x00009286 =>
	(ExifTag::UserComment, "User comment", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_encoded_string),

	0x0000a000 =>
	(ExifTag::FlashPixVersion, "Flashpix version", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_ascii),

	0x0000a001 =>
	(ExifTag::ColorSpace, "Color space", "none",
	IfdFormat::U16, 1, 1, color_space),

	0x0000a004 =>
	(ExifTag::RelatedSoundFile, "Related sound file", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0000a20b => (ExifTag::FlashEnergy, "Flash energy", "BCPS",
	IfdFormat::URational, 1, 1, flash_energy),

	0x0000a20e =>
	(ExifTag::FocalPlaneXResolution, "Focal plane X resolution", "@FocalPlaneResolutionUnit",
	IfdFormat::URational, 1, 1, rational_value),

	0x0000a20f =>
	(ExifTag::FocalPlaneYResolution, "Focal plane Y resolution", "@FocalPlaneResolutionUnit",
	IfdFormat::URational, 1, 1, rational_value),

	0x0000a210 =>
	(ExifTag::FocalPlaneResolutionUnit, "Focal plane resolution unit", "none",
	IfdFormat::U16, 1, 1, resolution_unit),

	0x0000a214 =>
	(ExifTag::SubjectLocation, "Subject location", "X,Y",
	IfdFormat::U16, 2, 2, subject_location),

	// TODO check if rational as decimal value is the best for this one
	0x0000a215 =>
	(ExifTag::ExposureIndex, "Exposure index", "EI",
	IfdFormat::URational, 1, 1, rational_value),

	0x0000a217 =>
	(ExifTag::SensingMethod, "Sensing method", "none",
	IfdFormat::U16, 1, 1, sensing_method),

	0x0000a300 =>
	(ExifTag::FileSource, "File source", "none",
	IfdFormat::Undefined, 1, 1, file_source),

	0x0000a301 =>
	(ExifTag::SceneType, "Scene type", "none",
	IfdFormat::Undefined, 1, 1, scene_type),

	0x0000a302 =>
	(ExifTag::CFAPattern, "CFA Pattern", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_u8),

	0x0000a401 =>
	(ExifTag::CustomRendered, "Custom rendered", "none",
	IfdFormat::U16, 1, 1, custom_rendered),

	0x0000a402 =>
	(ExifTag::ExposureMode, "Exposure mode", "none",
	IfdFormat::U16, 1, 1, exposure_mode),

	0x0000a403 =>
	(ExifTag::WhiteBalanceMode, "White balance mode", "none",
	IfdFormat::U16, 1, 1, white_balance_mode),

	0x0000a404 =>
	(ExifTag::DigitalZoomRatio, "Digital zoom ratio", "none",
	IfdFormat::URational, 1, 1, rational_value),

	0x0000a405 =>
	(ExifTag::FocalLengthIn35mmFilm, "Equivalent focal length in 35mm", "mm",
	IfdFormat::U16, 1, 1, focal_length_35),

	0x0000a406 =>
	(ExifTag::SceneCaptureType, "Scene capture type", "none",
	IfdFormat::U16, 1, 1, scene_capture_type),

	0x0000a407 =>
	(ExifTag::GainControl, "Gain control", "none",
	IfdFormat::U16, 1, 1, gain_control),

	0x0000a408 =>
	(ExifTag::Contrast, "Contrast", "none",
	IfdFormat::U16, 1, 1, contrast),

	0x0000a409 =>
	(ExifTag::Saturation, "Saturation", "none",
	IfdFormat::U16, 1, 1, saturation),

	0x0000a40a =>
	(ExifTag::Sharpness, "Sharpness", "none",
	IfdFormat::U16, 1, 1, sharpness),

	0x0000a432 =>
	(ExifTag::LensSpecification, "Lens specification", "none",
	IfdFormat::URational, 4, 4, lens_spec),

	0x0000a433 =>
	(ExifTag::LensMake, "Lens manufacturer", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x0000a434 =>
	(ExifTag::LensModel, "Lens model", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	// collaborate if you have any idea how to interpret this
	0x0000a40b =>
	(ExifTag::DeviceSettingDescription, "Device setting description", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_blob),

	0x0000a40c =>
	(ExifTag::SubjectDistanceRange, "Subject distance range", "none",
	IfdFormat::U16, 1, 1, subject_distance_range),

	0x0000a420 =>
	(ExifTag::ImageUniqueID, "Image unique ID", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),
		
	0x00000 =>
	(ExifTag::GPSVersionID, "GPS version ID", "none",
	IfdFormat::U8, 4, 4, strpass),

	0x00001 =>
	(ExifTag::GPSLatitudeRef, "GPS latitude ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00002 =>
	(ExifTag::GPSLatitude, "GPS latitude", "D/M/S",
	IfdFormat::URational, 3, 3, dms),

	0x00003 =>
	(ExifTag::GPSLongitudeRef, "GPS longitude ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00004 =>
	(ExifTag::GPSLongitude, "GPS longitude", "D/M/S",
	IfdFormat::URational, 3, 3, dms),

	0x00005 =>
	(ExifTag::GPSAltitudeRef, "GPS altitude ref", "none",
	IfdFormat::U8, 1, 1, gps_alt_ref),

	0x00006 =>
	(ExifTag::GPSAltitude, "GPS altitude", "m",
	IfdFormat::URational, 1, 1, meters),

	0x00007 =>
	(ExifTag::GPSTimeStamp, "GPS timestamp", "UTC time",
	IfdFormat::URational, 3, 3, gpstimestamp),

	0x00008 => (ExifTag::GPSSatellites, "GPS satellites", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00009 => (ExifTag::GPSStatus, "GPS status", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsstatus),

	0x0000a => (ExifTag::GPSMeasureMode, "GPS measure mode", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsmeasuremode),

	0x0000b =>
	(ExifTag::GPSDOP, "GPS Data Degree of Precision (DOP)", "none",
	IfdFormat::URational, 1, 1, rational_value),

	0x0000c =>
	(ExifTag::GPSSpeedRef, "GPS speed ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsspeedref),

	0x0000d =>
	(ExifTag::GPSSpeed, "GPS speed", "@GPSSpeedRef",
	IfdFormat::URational, 1, 1, gpsspeed),

	0x0000e =>
	(ExifTag::GPSTrackRef, "GPS track ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsbearingref),

	0x0000f =>
	(ExifTag::GPSTrack, "GPS track", "deg",
	IfdFormat::URational, 1, 1, gpsbearing),

	0x000010 =>
	(ExifTag::GPSImgDirectionRef, "GPS image direction ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsbearingref),

	0x000011 =>
	(ExifTag::GPSImgDirection, "GPS image direction", "deg",
	IfdFormat::URational, 1, 1, gpsbearing),

	0x000012 =>
	(ExifTag::GPSMapDatum, "GPS map datum", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x000013 =>
	(ExifTag::GPSDestLatitudeRef, "GPS destination latitude ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x000014 =>
	(ExifTag::GPSDestLatitude, "GPS destination latitude", "D/M/S",
	IfdFormat::URational, 3, 3, dms),

	0x000015 =>
	(ExifTag::GPSDestLongitudeRef, "GPS destination longitude ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x000016 =>
	(ExifTag::GPSDestLongitude, "GPS destination longitude", "D/M/S",
	IfdFormat::URational, 3, 3, dms),

	0x000017 =>
	(ExifTag::GPSDestBearingRef, "GPS destination bearing ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsbearingref),

	0x000018 =>
	(ExifTag::GPSDestBearing, "GPS destination bearing", "deg",
	IfdFormat::URational, 1, 1, gpsbearing),

	0x000019 =>
	(ExifTag::GPSDestDistanceRef, "GPS destination distance ref", "none",
	IfdFormat::Ascii, -1i32, -1i32, gpsdestdistanceref),

	0x00001a =>
	(ExifTag::GPSDestDistance, "GPS destination distance", "@GPSDestDistanceRef",
	IfdFormat::URational, 1, 1, gpsdestdistance),

	0x00001b =>
	(ExifTag::GPSProcessingMethod, "GPS processing method", "none",
	IfdFormat::Undefined, -1i32, -1i32, undefined_as_encoded_string),

	0x00001c => (ExifTag::GPSAreaInformation, "GPS area information",
	"none", IfdFormat::Undefined, -1i32, -1i32, undefined_as_encoded_string),

	0x00001d =>
	(ExifTag::GPSDateStamp, "GPS date stamp", "none",
	IfdFormat::Ascii, -1i32, -1i32, strpass),

	0x00001e =>
	(ExifTag::GPSDifferential, "GPS differential", "none",
	IfdFormat::U16, 1, 1, gpsdiff),

	_ =>
	(ExifTag::UnknownToMe, "Unknown to this library, or manufacturer-specific", "Unknown unit",
	IfdFormat::Unknown, -1i32, -1i32, nop)

	}
}

use super::types::*;
use super::exifreadable::*;

/// Convert a numeric tag into EXIF tag and yiels info about the tag
pub fn tag_to_exif(f: u16) -> (ExifTag, &'static str, &'static str, IfdFormat, i32, i32,
						fn(&TagValue, s: &String) -> String)
{
	match f {

	0x010e =>
	(ExifTag::ImageDescription, "Image Description", "none", IfdFormat::Str,
	-1i32, -1i32, strpass),

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

	0x8822 => (ExifTag::ExposureProgram, "Exposure program", "none",
	IfdFormat::U16, 1, 1, exposure_program),

	0x8824 => (ExifTag::SpectralSensitivity, "Spectral sensitivity", "", IfdFormat::Str, -1i32, -1i32, nop),

	// FIXME 
	// EPX
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

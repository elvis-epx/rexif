use super::rational::*;
use std::cell::RefCell;
use std::result::Result;

#[derive(Clone)]
pub struct ExifData {
	pub file: String,
	pub size: usize,
	pub mime: String,
	pub entries: Vec<ExifEntry>,
}

#[derive(Copy, Clone)]
pub enum ExifErrorKind {
	FileOpenError,
	FileSeekError,
	FileReadError,
	FileTypeUnknown,
	JpegWithoutExif,
	TiffTruncated,
	TiffBadPreamble,
	IfdTruncated,
	ExifIfdTruncated,
	ExifIfdEntryNotFound,
}

#[derive(Clone)]
pub struct ExifError {
	pub kind: ExifErrorKind,
	pub extra: String
}

#[derive(Clone)]
pub struct IfdEntry {
	pub tag: u16,
	pub format: IfdFormat,
	pub count: u32,
	pub data: Vec<u8>,
	pub ifd_data: Vec<u8>,
	pub ext_data: Vec<u8>,
	pub le: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ExifTag {
	UnknownToMe = 0xffff,
	ImageDescription = 0x010e,
	Make = 0x010f,
	Model = 0x0110,
	Orientation = 0x0112,
	XResolution = 0x011a,
	YResolution = 0x011b,
	ResolutionUnit = 0x0128,
	Software = 0x0131,
	DateTime = 0x0132,
	WhitePoint = 0x013e,
	PrimaryChromaticities = 0x013f,
	YCbCrCoefficients = 0x0211,
	ReferenceBlackWhite = 0x0214,
	Copyright = 0x8298,
	ExifOffset = 0x8769,
	GPSOffset = 0x8825,

	ExposureTime = 0x829a,
	FNumber = 0x829d,
	ExposureProgram = 0x8822,
	SpectralSensitivity = 0x8824,
	ISOSpeedRatings = 0x8827,
	OECF = 0x8828,
	ExifVersion = 0x9000,
	DateTimeOriginal = 0x9003,
	DateTimeDigitized = 0x9004,
	ComponentsConfiguration = 0x9101,
	CompressedBitsPerPixel = 0x9102,
	ShutterSpeedValue = 0x9201,
	ApertureValue = 0x9202,
	BrightnessValue = 0x9203,
	ExposureBiasValue = 0x9204,
	MaxApertureValue = 0x9205,
	SubjectDistance = 0x9206,
	MeteringMode = 0x9207,
	LightSource = 0x9208,
	Flash = 0x9209,
	FocalLength = 0x920a,
	SubjectArea = 0x9214,
	MakerNote = 0x927c,
	UserComment = 0x9286,
	FlashPixVersion = 0xa000,
	ColorSpace = 0xa001,
	RelatedSoundFile = 0xa004,
	FlashEnergy = 0xa20b,
	FocalPlaneXResolution = 0xa20e,
	FocalPlaneYResolution = 0xa20f,
	FocalPlaneResolutionUnit = 0xa210,
	SubjectLocation = 0xa214,
	ExposureIndex = 0xa215,
	SensingMethod = 0xa217,
	FileSource = 0xa300,
	SceneType = 0xa301,
	CFAPattern = 0xa302,
	CustomRendered = 0xa401,
	ExposureMode = 0xa402,
	WhiteBalanceMode = 0xa403,
	DigitalZoomRatio = 0xa404,
	FocalLengthIn35mmFilm = 0xa405,
	SceneCaptureType = 0xa406,
	GainControl = 0xa407,
	Contrast = 0xa408,
	Saturation = 0xa409,
	Sharpness = 0xa40a,
	DeviceSettingDescription = 0xa40b,
	SubjectDistanceRange = 0xa40c,
	ImageUniqueID = 0xa420,
		
	GPSVersionID = 0x0,
	GPSLatitudeRef = 0x1,
	GPSLatitude = 0x2,
	GPSLongitudeRef = 0x3,
	GPSLongitude = 0x4,
	GPSAltitudeRef = 0x5,
	GPSAltitude = 0x6,
	GPSTimeStamp = 0x7,
	GPSSatellites = 0x8,
	GPSStatus = 0x9,
	GPSMeasureMode = 0xa,
	GPSDOP = 0xb,
	GPSSpeedRef = 0xc,
	GPSSpeed = 0xd,
	GPSTrackRef = 0xe,
	GPSTrack = 0xf,
	GPSImgDirectionRef = 0x10,
	GPSImgDirection = 0x11,
	GPSMapDatum = 0x12,
	GPSDestLatitudeRef = 0x13,
	GPSDestLatitude = 0x14,
	GPSDestLongitudeRef = 0x15,
	GPSDestLongitude = 0x16,
	GPSDestBearingRef = 0x17,
	GPSDestBearing = 0x18,
	GPSDestDistanceRef = 0x19,
	GPSDestDistance = 0x1a,
	GPSProcessingMethod = 0x1b,
	GPSAreaInformation = 0x1c,
	GPSDateStamp = 0x1d,
	GPSDifferential = 0x1e,
}

#[derive(Copy, Clone, PartialEq)]
pub enum IfdFormat {
	Unknown = 0,
	U8 = 1,
	Str = 2,
	U16 = 3,
	U32 = 4,
	URational = 5,
	I8 = 6,
	Undefined = 7, // u8
	I16 = 8,
	I32 = 9,
	IRational = 10,
	F32 = 11,
	F64 = 12,
}

#[derive(Clone)]
pub struct ExifEntry {
	pub ifd: IfdEntry,
	pub tag: ExifTag,
	pub value: TagValue,
	pub unit: String,
	pub tag_readable: String,
	pub value_readable: String,
	pub value_more_readable: String,
}

#[derive(Clone)]
pub enum TagValue {
	U8(Vec<u8>),
	Str(String),
	U16(Vec<u16>),
	U32(Vec<u32>),
	URational(Vec<URational>),
	I8(Vec<i8>),
	Undefined(Vec<u8>),
	I16(Vec<i16>),
	I32(Vec<i32>),
	IRational(Vec<IRational>),
	F32(Vec<f32>),
	F64(Vec<f64>),
	Unknown(Vec<u8>),
}

pub type ExifResult = Result<RefCell<ExifData>, ExifError>;
pub type InExifResult = Result<(), ExifError>;

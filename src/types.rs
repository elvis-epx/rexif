use super::rational::*;
use std::cell::RefCell;
use std::result::Result;

/// Top-level structure that contains all data inside an image
#[derive(Clone)]
pub struct ExifData {
	/// File name, if a file was submitted for parsing, otherwise empty
	pub file: String,
	/// Image file size, in bytes
	pub size: usize,
	/// MIME type of the parsed image. image/jpeg, image/tiff, or empty if unrecognized.
	pub mime: String,
	/// List of EXIF entries found in the image
	pub entries: Vec<ExifEntry>,
}

/// Possible fatal errors that may happen when an image is parsed.
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

/// EXIF parsing error type
#[derive(Clone)]
pub struct ExifError {
	/// The general kind of the error that aborted the parsing
	pub kind: ExifErrorKind,
	/// Extra context info about the error, when available
	pub extra: String
}

/// Structure that represents a parsed IFD entry of a TIFF image
#[derive(Clone)]
pub struct IfdEntry {
	/// IFD tag value, may or not be an EXIF tag
	pub tag: u16,
	/// IFD data format
	pub format: IfdFormat,
	/// Number of items, each one in the data format specified by format
	pub count: u32,
	/// Raw data as a vector of bytes. Length is sizeof(format) * count.
	/// It is a copy of either ifd_data or ext_data, depending on size.
	pub data: Vec<u8>,
	/// Raw data contained within the IFD structure. If count * sizeof(format) >= 4,
	/// this item contains the offset where the actual data can be found
	pub ifd_data: Vec<u8>,
	/// Raw data contained outside of the IFD structure, if it did not fit within the IFD structure
	pub ext_data: Vec<u8>,
	/// If true, integer and offset formats must be parsed from raw data as little-endian
	/// If false, integer and offset formats must be parsed from raw data as big-endian
	pub le: bool,
}

/// Enumeration that represents recognized EXIF tags found in TIFF IFDs
#[derive(Copy, Clone, PartialEq)]
pub enum ExifTag {
	/// Tag not recognized are partially parsed. The client may still try to interpret
	/// the tag by reading into the IfdFormat structure.
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

/// Enumeration that represents the possible data formats of an IFD entry
#[derive(Copy, Clone, PartialEq)]
pub enum IfdFormat {
	Unknown = 0,
	U8 = 1,
	Ascii = 2,
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

/// Structure that represents a parsed EXIF tag.
#[derive(Clone)]
pub struct ExifEntry {
	/// Low-level IFD entry that contains the EXIF tag. The client may look into this
	/// structure if tag is UnknownToMe, or to get the tag's raw data.
	pub ifd: IfdEntry,
	/// EXIF tag type as an enumeration. If UnknownToMe, the crate does not know the
	/// tag in detail, and parsing will be incomplete. The client may read into the
	/// ifd member to discover more about the unparsed tag.
	pub tag: ExifTag,
	/// EXIF tag value as an enumeration or "variant"
	pub value: TagValue,
	/// Unit of the value, if applicable. If tag is UnknownToMe, unit will be empty.
	/// If the tag is parsed and it is unitless, it will be equal to "none".
	///
	/// Note that
	/// unit refers to the contents of 'value', not the readable string. For example,
	/// a GPS latitude is a triplet of rational values, so unit is D/M/S, even though
	/// the value_more_readable string contains a single string with the three parts
	/// combined.
	pub unit: String,
	/// Human-readable name of the tag, for debugging and simple listing purposes
	pub tag_readable: String,
	/// Human-readable, bare, version of the value. It is a bare value because e.g.
	/// enumerations are not interpreted. Even if tag is UnknownToMe, this
	/// member contains a representation of data found inside IFD entry.
	pub value_readable: String,
	/// Human-readable, interpreted and "pretty" version of the value. It is "pretty"
	/// because e.g. enumerations are interpreted and values are accompanied by unit.
	/// If tag is UnknownToMe,
	/// this member contains the same string as value_readable.
	pub value_more_readable: String,
}

/// Tag value enumeration. It works as a variant type. Each value is
/// actually a vector because many EXIF tags are collections of values.
/// Exif tags with single values are represented as single-item vectors.
#[derive(Clone)]
pub enum TagValue {
	U8(Vec<u8>),
	Ascii(String),
	U16(Vec<u16>),
	U32(Vec<u32>),
	URational(Vec<URational>),
	I8(Vec<i8>),
	/// Undefined value has a "little endian" boolean parameter.
	/// If true, and if buffer contents have some sort of internal structure,
	/// they should be interpreted as LE.
	Undefined(Vec<u8>, bool),
	I16(Vec<i16>),
	I32(Vec<i32>),
	IRational(Vec<IRational>),
	F32(Vec<f32>),
	F64(Vec<f64>),
	/// Unknown value has a "little endian" boolean parameter.
	/// If true, and if buffer contents have some sort of internal structure,
	/// they should be interpreted as LE.
	Unknown(Vec<u8>, bool),
}

/// Type returned by image parsing
pub type ExifResult = Result<RefCell<ExifData>, ExifError>;

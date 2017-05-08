use super::rational::*;
use std::fmt;
use std::result::Result;
use std::io;

/// Top-level structure that contains all parsed metadata inside an image
#[derive(Debug)]
pub struct ExifData {
	/// MIME type of the parsed image. It may be "image/jpeg", "image/tiff", or empty if unrecognized.
	pub mime: String,
	/// Collection of EXIF entries found in the image
	pub entries: Vec<ExifEntry>,
}

/// Possible fatal errors that may happen when an image is parsed.
#[derive(Debug)]
pub enum ExifError {
	IoError(io::Error),
	FileTypeUnknown,
	JpegWithoutExif(String),
	TiffTruncated,
	TiffBadPreamble(String),
	IfdTruncated,
	ExifIfdTruncated(String),
	ExifIfdEntryNotFound,
}

/// Structure that represents a parsed IFD entry of a TIFF image
#[derive(Clone, Debug)]
pub struct IfdEntry {
	/// Namespace of the entry. Standard is a tag found in normal TIFF IFD structure,
	/// other namespaces are entries found e.g. within MarkerNote blobs that are
	/// manufacturer-specific.
	pub namespace: Namespace,
	/// IFD tag value, may or not be an EXIF tag
	pub tag: u16,
	/// IFD data format
	pub format: IfdFormat,
	/// Number of items, each one in the data format specified by format
	pub count: u32,
	/// Raw data as a vector of bytes. Length is sizeof(format) * count.
	/// Depending on its size, it came from different parts of the image file.
	pub data: Vec<u8>,
	/// If true, integer and offset formats must be parsed from raw data as little-endian.
	/// If false, integer and offset formats must be parsed from raw data as big-endian.
	///
	/// It is important to have 'endianess' per IFD entry, because some manufacturer-specific
	/// entries may have fixed endianess (regardeless of TIFF container's general endianess).
	pub le: bool,
}

/// Enumeration that represent EXIF tag namespaces. Namespaces exist to
/// accomodate future parsing of the manufacturer-specific tags embedded within
/// the MarkerNote tag.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Namespace {
	Standard = 0x0000,
	Nikon = 0x0001,
	Canon = 0x0002,
}

/// Enumeration that represents recognized EXIF tags found in TIFF IFDs.
///
/// Items can be cast to u32 in order to get the namespace (most significant word)
/// and tag code (least significant word). The tag code matches the Exif, or the
/// Makernote standard, depending on the namespace that the tag belongs to.
///
/// On the other hand, the namespace code is arbitrary, it only matches
/// the `Namespace` enumeration. The namespace is 0 for standard Exif tags.
/// The non-standard namespaces exist to accomodate future parsing of the
/// MarkerNote tag, that contains embedded manufacturer-specific tags.
#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum ExifTag {
	ImageDescription = 0x010e,
	Make = 0x010f,
	Model = 0x0110,
	Orientation = 0x0112,
	XResolution = 0x011a,
	YResolution = 0x011b,
	ResolutionUnit = 0x0128,
	Software = 0x0131,
	DateTime = 0x0132,
	HostComputer = 0x013c,
	WhitePoint = 0x013e,
	PrimaryChromaticities = 0x013f,
	YCbCrCoefficients = 0x0211,
	ReferenceBlackWhite = 0x0214,
	Copyright = 0x8298,
	ExifOffset = 0x8769,
	GPSInfo = 0x8825,

	ExposureTime = 0x829a,
	FNumber = 0x829d,
	ExposureProgram = 0x8822,
	SpectralSensitivity = 0x8824,
	PhotographicSensitivity = 0x8827,
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
	FlashpixVersion = 0xa000,
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
	WhiteBalance = 0xa403,
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
	LensSpecification = 0xa432,
	LensMake = 0xa433,
	LensModel = 0xa434,

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

impl Eq for ExifTag {}


impl fmt::Display for ExifTag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			ExifTag::ImageDescription => "Image Description",
			ExifTag::Make => "Manufacturer",
			ExifTag::HostComputer => "Host computer",
			ExifTag::Model => "Model",
			ExifTag::Orientation => "Orientation",
			ExifTag::XResolution => "X Resolution",
			ExifTag::YResolution => "Y Resolution",
			ExifTag::ResolutionUnit => "Resolution Unit",
			ExifTag::Software => "Software",
			ExifTag::DateTime => "Image date",
			ExifTag::WhitePoint => "White Point",
			ExifTag::PrimaryChromaticities => "Primary Chromaticities",
			ExifTag::YCbCrCoefficients => "YCbCr Coefficients",
			ExifTag::ReferenceBlackWhite => "Reference Black/White",
			ExifTag::Copyright => "Copyright",
			ExifTag::ExifOffset => "This image has an Exif SubIFD",
			ExifTag::GPSInfo => "This image has a GPS SubIFD",
			ExifTag::ExposureTime => "Exposure time",
			ExifTag::FNumber => "Aperture",
			ExifTag::ExposureProgram => "Exposure program",
			ExifTag::SpectralSensitivity => "Spectral sensitivity",
			ExifTag::PhotographicSensitivity => "ISO speed ratings",
			ExifTag::OECF => "OECF",
			ExifTag::ExifVersion => "Exif version",
			ExifTag::DateTimeOriginal => "Date of original image",
			ExifTag::DateTimeDigitized => "Date of image digitalization",
			ExifTag::ShutterSpeedValue => "Shutter speed",
			ExifTag::ApertureValue => "Aperture value",
			ExifTag::BrightnessValue => "Brightness value",
			ExifTag::ExposureBiasValue => "Exposure bias value",
			ExifTag::MaxApertureValue => "Maximum aperture value",
			ExifTag::SubjectDistance => "Subject distance",
			ExifTag::MeteringMode => "Meteting mode",
			ExifTag::LightSource => "Light source",
			ExifTag::Flash => "Flash",
			ExifTag::FocalLength => "Focal length",
			ExifTag::SubjectArea => "Subject area",
			ExifTag::MakerNote => "Maker note",
			ExifTag::UserComment => "User comment",
			ExifTag::FlashpixVersion => "Flashpix version",
			ExifTag::ColorSpace => "Color space",
			ExifTag::FlashEnergy => "Flash energy",
			ExifTag::RelatedSoundFile => "Related sound file",
			ExifTag::FocalPlaneXResolution => "Focal plane X resolution",
			ExifTag::FocalPlaneYResolution => "Focal plane Y resolution",
			ExifTag::FocalPlaneResolutionUnit => "Focal plane resolution unit",
			ExifTag::SubjectLocation => "Subject location",
			ExifTag::ExposureIndex => "Exposure index",
			ExifTag::SensingMethod => "Sensing method",
			ExifTag::FileSource => "File source",
			ExifTag::SceneType => "Scene type",
			ExifTag::CFAPattern => "CFA Pattern",
			ExifTag::CustomRendered => "Custom rendered",
			ExifTag::ExposureMode => "Exposure mode",
			ExifTag::WhiteBalance => "White balance mode",
			ExifTag::DigitalZoomRatio => "Digital zoom ratio",
			ExifTag::FocalLengthIn35mmFilm => "Equivalent focal length in 35mm",
			ExifTag::SceneCaptureType => "Scene capture type",
			ExifTag::GainControl => "Gain control",
			ExifTag::Contrast => "Contrast",
			ExifTag::Saturation => "Saturation",
			ExifTag::Sharpness => "Sharpness",
			ExifTag::LensSpecification => "Lens specification",
			ExifTag::LensMake => "Lens manufacturer",
			ExifTag::LensModel => "Lens model",
			ExifTag::DeviceSettingDescription => "Device setting description",
			ExifTag::SubjectDistanceRange => "Subject distance range",
			ExifTag::ImageUniqueID => "Image unique ID",
			ExifTag::GPSVersionID => "GPS version ID",
			ExifTag::GPSLatitudeRef => "GPS latitude ref",
			ExifTag::GPSLatitude => "GPS latitude",
			ExifTag::GPSLongitudeRef => "GPS longitude ref",
			ExifTag::GPSLongitude => "GPS longitude",
			ExifTag::GPSAltitudeRef => "GPS altitude ref",
			ExifTag::GPSAltitude => "GPS altitude",
			ExifTag::GPSTimeStamp => "GPS timestamp",
			ExifTag::GPSSatellites => "GPS satellites",
			ExifTag::GPSStatus => "GPS status",
			ExifTag::GPSMeasureMode => "GPS measure mode",
			ExifTag::GPSDOP => "GPS Data Degree of Precision (DOP)",
			ExifTag::GPSSpeedRef => "GPS speed ref",
			ExifTag::GPSSpeed => "GPS speed",
			ExifTag::GPSTrackRef => "GPS track ref",
			ExifTag::GPSTrack => "GPS track",
			ExifTag::GPSImgDirectionRef => "GPS image direction ref",
			ExifTag::GPSImgDirection => "GPS image direction",
			ExifTag::GPSMapDatum => "GPS map datum",
			ExifTag::GPSDestLatitudeRef => "GPS destination latitude ref",
			ExifTag::GPSDestLatitude => "GPS destination latitude",
			ExifTag::GPSDestLongitudeRef => "GPS destination longitude ref",
			ExifTag::GPSDestLongitude => "GPS destination longitude",
			ExifTag::GPSDestBearingRef => "GPS destination bearing ref",
			ExifTag::GPSDestBearing => "GPS destination bearing",
			ExifTag::GPSDestDistanceRef => "GPS destination distance ref",
			ExifTag::GPSDestDistance => "GPS destination distance",
			ExifTag::GPSProcessingMethod => "GPS processing method",
			ExifTag::GPSAreaInformation => "GPS area information",
			ExifTag::GPSDateStamp => "GPS date stamp",
			ExifTag::GPSDifferential => "GPS differential",
		})
	}
}

/// Enumeration that represents the possible data formats of an IFD entry.
///
/// Any enumeration item can be cast to u16 to get the low-level format code
/// as defined by the TIFF format.
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// Enumeration that stores the tag value of an IFD entry. If this crate recognises a tag, it will
/// be stored as an enumeration, otherwise its raw value will be stored so that the client may
/// interpret it themselves.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IfdTag {
	Unknown(u16),
	Exif(ExifTag),
}

/// Structure that represents a parsed EXIF tag.
#[derive(Clone, Debug)]
pub struct ExifEntry {
	/// Namespace of the tag. If Standard (0x0000), it is an EXIF tag defined in the
	/// official standard. Other namespaces accomodate manufacturer-specific tags that
	/// may be embedded in MarkerNote blob tag.
	pub namespace: Namespace,
	/// Low-level IFD entry that contains the EXIF tag. The client may look into this
	/// structure to get tag's raw data, or to parse the tag herself if `tag` is `Unknown`.
	pub ifd: IfdEntry,
	/// EXIF tag type as an enumeration. If `Unknown(x)`, the crate did not know the
	/// tag in detail, and parsing will be incomplete. The client may read into
	/// `x` to discover more about the unparsed tag.
	pub tag: IfdTag,
	/// EXIF tag value as an enumeration. Behaves as a "variant" value
	pub value: TagValue,
	/// Human-readable and "pretty" version of `value`.
	/// Enumerations and tuples are interpreted and combined. If `value`
	/// has a unit, it is also added.
	/// If tag is `Unknown`,
	/// this member contains the same string as obtained by formatting `value`.
	pub value_more_readable: String,
}

/// Tag value enumeration. It works as a variant type. Each value is
/// actually a vector because many EXIF tags are collections of values.
/// Exif tags with single values are represented as single-item vectors.
#[derive(Clone, Debug)]
pub enum TagValue {
	/// Array of unsigned byte integers
	U8(Vec<u8>),
	/// ASCII string. (The standard specifies 7-bit ASCII, but this parser accepts UTF-8 strings.)
	Ascii(String),
	U16(Vec<u16>),
	U32(Vec<u32>),
	/// Array of `URational` structures (tuples with integer numerator and denominator)
	URational(Vec<URational>),
	I8(Vec<i8>),
	/// Array of bytes with opaque internal structure. Used by manufacturer-specific
	/// tags, SIG-specific tags, tags that contain Unicode (UCS-2) or Japanese (JIS)
	/// strings (i.e. strings that are not 7-bit-clean), tags that contain
	/// dissimilar or variant types, etc.
	///
	/// This item has a "little endian"
	/// boolean parameter that reports the whole TIFF's endianness.
	/// Any sort of internal structure that is sensitive to endianess
	/// should be interpreted accordignly to this parameter (true=LE, false=BE).
	Undefined(Vec<u8>, bool),
	I16(Vec<i16>),
	I32(Vec<i32>),
	/// Array of `IRational` structures (tuples with signed integer numerator and denominator)
	IRational(Vec<IRational>),
	/// Array of IEEE 754 floating-points
	F32(Vec<f32>),
	/// Array of IEEE 754 floating-points
	F64(Vec<f64>),
	/// Array of bytes with unknown internal structure.
	/// This is different from `Undefined` because `Undefined` is actually a specified
	/// format, while `Unknown` is an unexpected format type. A tag of `Unknown` format
	/// is most likely a corrupted tag.
	///
	/// This variant has a "little endian"
	/// boolean parameter that reports the whole TIFF's endianness.
	/// Any sort of internal structure that is sensitive to endianess
	/// should be interpreted accordignly to this parameter (true=LE, false=BE).
	Unknown(Vec<u8>, bool),
	/// Type that could not be parsed due to some sort of error (e.g. buffer too
	/// short for the count and type size). Variant contains raw data, LE/BE,
	/// format (as u16) and count.
	Invalid(Vec<u8>, bool, u16, u32)
}

/// Type returned by image file parsing
pub type ExifResult = Result<ExifData, ExifError>;

/// Type resturned by lower-level parsing functions
pub type ExifEntryResult = Result<Vec<ExifEntry>, ExifError>;

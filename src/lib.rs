use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
use std::result::Result;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::cell::RefCell;

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
	YCbCrPositioning = 0x213,
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

#[derive(Copy, Clone)]
pub struct URational {
	pub numerator: u32,
	pub denominator: u32,
}

impl URational {
	fn value(&self) -> f64 {
		(self.numerator as f64) / (self.denominator as f64)
	}
}

impl Display for URational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}

#[derive(Copy, Clone)]
pub struct IRational {
	pub numerator: i32,
	pub denominator: i32,
}

impl IRational {
	fn value(&self) -> f64 {
		(self.numerator as f64) / (self.denominator as f64)
	}
}

impl Display for IRational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
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

fn to_ifdformat(n: u16) -> IfdFormat
{
	match n {
		1 => IfdFormat::U8,
		2 => IfdFormat::Str,
		3 => IfdFormat::U16,
		4 => IfdFormat::U32,
		5 => IfdFormat::URational,
		6 => IfdFormat::I8,
		7 => IfdFormat::Undefined,
		8 => IfdFormat::I16,
		9 => IfdFormat::I32,
		10 => IfdFormat::IRational,
		11 => IfdFormat::F32,
		12 => IfdFormat::F64,
		_ => IfdFormat::Unknown,
	}
}

impl IfdEntry {
	fn data_as_offset(&self) -> usize {
		read_u32(self.le, &(self.ifd_data[0..4])) as usize
	}

	fn size(&self) -> u8
	{
		match self.format {
			IfdFormat::U8 => 1,
			IfdFormat::Str => 1,
			IfdFormat::U16 => 2,
			IfdFormat::U32 => 4,
			IfdFormat::URational => 8,
			IfdFormat::I8 => 1,
			IfdFormat::Undefined => 1,
			IfdFormat::I16 => 2,
			IfdFormat::I32 => 4,
			IfdFormat::IRational => 8,
			IfdFormat::F32 => 4,
			IfdFormat::F64 => 8,
			IfdFormat::Unknown => 1,
		}
	}

	fn length(&self) -> usize
	{
		(self.size() as usize) * (self.count as usize)
	}

	fn in_ifd(&self) -> bool
	{
		self.length() <= 4
	}

	fn copy_data(&mut self, contents: &[u8]) -> bool
	{
		if self.in_ifd() {
			// the 4 bytes from IFD have all data
			self.data = self.ifd_data.clone();
			return true;
		}

		let offset = self.data_as_offset();
		if contents.len() < (offset + self.length()) {
			// println!("EXIF data block goes beyond EOF");
			return false;
		}

		let ext_data = &contents[offset..(offset + self.length())];
		self.ext_data.clear();	
		self.ext_data.extend(ext_data);
		self.data = self.ext_data.clone();
		return true;
	}
}

impl ExifError {
	fn readable(&self) -> &str {
		let msg = match self.kind {
			ExifErrorKind::FileOpenError => "File could not be opened",
			ExifErrorKind::FileSeekError => "File could not be seeked",
			ExifErrorKind::FileReadError => "File could not be read",
			ExifErrorKind::FileTypeUnknown => "File type unknown",
			ExifErrorKind::JpegWithoutExif => "JPEG without EXIF section",
			ExifErrorKind::TiffTruncated => "TIFF truncated at start",
			ExifErrorKind::TiffBadPreamble => "TIFF with bad preamble",
			ExifErrorKind::IfdTruncated => "TIFF IFD truncated",
			ExifErrorKind::ExifIfdTruncated => "TIFF Exif IFD truncated",
			ExifErrorKind::ExifIfdEntryNotFound => "TIFF Exif IFD not found",
		};
		return msg;
	}
}

impl Error for ExifError {
	fn description(&self) -> &str {
		self.readable()
	}
}

impl Debug for ExifError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.readable(), self.extra)
	}
}

impl Display for ExifError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.readable(), self.extra)
	}
}

pub type ExifResult = Result<RefCell<ExifData>, ExifError>;
pub type InExifResult = Result<(), ExifError>;

/* Detect the type of an image contained in a byte buffer */
pub fn detect_type(contents: &Vec<u8>) -> &str
{
	if contents.len() < 11 {
		return "";
	}

	if contents[0] == 0xff && contents[1] == 0xd8 &&
			contents[2] == 0xff && // contents[3] == 0xe0 &&
			contents[6] == ('J' as u8) && contents[7] == ('F' as u8) &&
			contents[8] == ('I' as u8) && contents[9] == ('F' as u8) &&
			contents[10] == 0 {
		return "image/jpeg";
	}
	if contents[0] == ('I' as u8) && contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		return "image/tiff";
	}
	if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
		return "image/tiff";
	}

	return "";
}

/* Find the embedded TIFF in a JPEG image, that contains in turn the EXIF data */
pub fn find_embedded_tiff_in_jpeg(contents: &Vec<u8>) -> (usize, usize, String)
{
	let mut err = "Scan past EOF and no EXIF found".to_string();
	
	{
	let mut offset = 2 as usize;
	let mut size: usize;

	while offset < contents.len() {
		if contents.len() < (offset + 4) {
			err = "JPEG truncated in marker header".to_string();
			break;
		}

		let marker: u16 = (contents[offset] as u16) * 256 + (contents[offset + 1] as u16);

		if marker < 0xff00 {
			err = format!("Invalid marker {:x}", marker);
			break;
		}

		offset += 2;
		size = (contents[offset] as usize) * 256 + (contents[offset + 1] as usize);

		if size < 2 {
			err = "JPEG marker size must be at least 2 (because of the size word)".to_string();
			break;
		}
		if contents.len() < (offset + size) {
			err = "JPEG truncated in marker body".to_string();
			break;
		}

		if marker == 0xffe1 {
			// Discard the size word
			offset += 2;
			size -= 2;

			if size < 6 {
				err = "EXIF preamble truncated".to_string();
				break;
			}

			if contents[offset + 0] != ('E' as u8) &&
					contents[offset + 1] != ('x' as u8) &&
					contents[offset + 2] != ('i' as u8) &&
					contents[offset + 3] != ('f' as u8) &&
					contents[offset + 4] != 0 &&
					contents[offset + 5] != 0 {
				err = "EXIF preamble unrecognized".to_string();
				break;
			}

			// Discard the 'Exif\0\0' preamble
			offset += 6;
			size -= 6;

			return (offset, size, "".to_string());
		}
		if marker == 0xffda {
			// last marker
			err = "Last mark found and no EXIF".to_string();
			break;
		}

		offset += size;
	}
	}

	return (0, 0, err);
}

/* Convert u8 to i8 */
fn read_i8(raw: u8) -> i8
{
	let mut u = raw as i16;
	if u >= 0x80 {
		u = u - 0x100;
	}
	return u as i8;
}

/* Read value from a stream of bytes */
fn read_u16(le: bool, raw: &[u8]) -> u16
{
	if le {
		(raw[1] as u16) * 256 + raw[0] as u16
	} else {
		(raw[0] as u16) * 256 + raw[1] as u16
	}
}

/* Read value from a stream of bytes */
fn read_i16(le: bool, raw: &[u8]) -> i16
{
	let mut u = read_u16(le, raw) as i32;
	if u >= 0x8000 {
		u = u - 0x10000;
	}
	return u as i16;
}

/* Read value from a stream of bytes */
fn read_u32(le: bool, raw: &[u8]) -> u32
{
	if le {
		((raw[3] as u32) << 24) + ((raw[2] as u32) << 16) +
		((raw[1] as u32) << 8) + raw[0] as u32
	} else {
		((raw[0] as u32) << 24) + ((raw[1] as u32) << 16) +
		((raw[2] as u32) << 8) + raw[3] as u32
	}
}

/* Read value from a stream of bytes */
fn read_i32(le: bool, raw: &[u8]) -> i32
{
	let mut u = read_u32(le, raw) as i64;
	if u >= 0x80000000 {
		u = u - 0x100000000;
	}
	return u as i32;
}

/* Read value from a stream of bytes */
fn read_f32(raw: &[u8]) -> f32
{
	let mut a = [0 as u8; 4];
	// idiot, but guarantees that transmute gets a 4-byte buffer
	for i in 0..4 {
		a[i] = raw[i];
	}
	// FIXME I am not sure that TIFF floating point can be cast this way for any given architecture
	// The ideal thing would be to read mantissa, exponent, etc. explicitly
	let f: f32 = unsafe { std::mem::transmute(a) }; 
	return f;
}

/* Read value from a stream of bytes */
fn read_f64(raw: &[u8]) -> f64
{
	let mut a = [0 as u8; 8];
	for i in 0..8 {
		a[i] = raw[i];
	}
	// FIXME I am not sure that TIFF floating point can be cast this way for any given architecture
	// The ideal thing would be to read mantissa, exponent, etc. explicitly
	let f: f64 = unsafe { std::mem::transmute(a) };
	return f;
}

/* Read value from a stream of bytes */
fn read_urational(le: bool, raw: &[u8]) -> URational
{
	let n = read_u32(le, &raw[0..4]);
	let d = read_u32(le, &raw[4..8]);
	return URational{numerator: n, denominator: d};
}

/* Read value from a stream of bytes */
fn read_irational(le: bool, raw: &[u8]) -> IRational
{
	let n = read_i32(le, &raw[0..4]);
	let d = read_i32(le, &raw[4..8]);
	return IRational{numerator: n, denominator: d};
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_i8_array(count: u32, raw: &[u8]) -> Vec<i8>
{
	let mut a = Vec::<i8>::new();
	for i in 0..count {
		a.push(read_i8(raw[i as usize]));
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_u16_array(le: bool, count: u32, raw: &[u8]) -> Vec<u16>
{
	let mut a = Vec::<u16>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_u16(le, &raw[offset..offset + 2]));
		offset += 2;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_i16_array(le: bool, count: u32, raw: &[u8]) -> Vec<i16>
{
	let mut a = Vec::<i16>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_i16(le, &raw[offset..offset + 2]));
		offset += 2;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_u32_array(le: bool, count: u32, raw: &[u8]) -> Vec<u32>
{
	let mut a = Vec::<u32>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_u32(le, &raw[offset..offset + 4]));
		offset += 4;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_i32_array(le: bool, count: u32, raw: &[u8]) -> Vec<i32>
{
	let mut a = Vec::<i32>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_i32(le, &raw[offset..offset + 4]));
		offset += 4;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_f32_array(count: u32, raw: &[u8]) -> Vec<f32>
{
	let mut a = Vec::<f32>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_f32(&raw[offset..offset + 4]));
		offset += 4;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_f64_array(count: u32, raw: &[u8]) -> Vec<f64>
{
	let mut a = Vec::<f64>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_f64(&raw[offset..offset + 8]));
		offset += 8;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_urational_array(le: bool, count: u32, raw: &[u8]) -> Vec<URational>
{
	let mut a = Vec::<URational>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_urational(le, &raw[offset..offset + 8]));
		offset += 8;
	}
	return a;
}

/* Read array from a stream of bytes. Caller must be sure of count and buffer size */
fn read_irational_array(le: bool, count: u32, raw: &[u8]) -> Vec<IRational>
{
	let mut a = Vec::<IRational>::new();
	let mut offset = 0;
	for _ in 0..count {
		a.push(read_irational(le, &raw[offset..offset + 8]));
		offset += 8;
	}
	return a;
}

fn numarray_to_string<T: Display>(numbers: &Vec<T>) -> String
{
	if numbers.len() < 1 {
		return "".to_string();
	} else if numbers.len() == 1 {
		return format!("{}", &numbers[0]);
	}

	let mut s = "".to_string();
	let mut first = true;
	for number in numbers {
		if !first {
			s = s + ", ";
		}
		first = false;
		let s2 = format!("{}", number);
		s = s + &s2;
	}

	return s;
}

fn tag_value(f: &IfdEntry) -> (TagValue, String)
{
	match f.format {
		IfdFormat::Str => {
			let s = String::from_utf8_lossy(&f.data[..]);
			let s = s.into_owned();
			(TagValue::Str(s.to_string()), s.to_string())
		},
		IfdFormat::U16 => {
			let a = read_u16_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::U16(a), b)
		},
		IfdFormat::I16 => {
			let a = read_i16_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I16(a), b)
		},
		IfdFormat::U8 => {
			let a = f.data.clone();
			let b = numarray_to_string(&a);
			(TagValue::U8(a), b)
		},
		IfdFormat::I8 => {
			let a = read_i8_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I8(a), b)
		},
		IfdFormat::U32 => {
			let a = read_u32_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::U32(a), b)
		},
		IfdFormat::I32 => {
			let a = read_i32_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::I32(a), b)
		},
		IfdFormat::F32 => {
			let a = read_f32_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::F32(a), b)
		},
		IfdFormat::F64 => {
			let a = read_f64_array(f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::F64(a), b)
		},
		IfdFormat::URational => {
			let a = read_urational_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::URational(a), b)
		},
		IfdFormat::IRational => {
			let a = read_irational_array(f.le, f.count, &f.data[..]);
			let b = numarray_to_string(&a);
			(TagValue::IRational(a), b)
		},

		IfdFormat::Undefined => (TagValue::Undefined(f.data.clone()),
					"<blob>".to_string()),

		_ => (TagValue::Unknown(f.data.clone()),
					"<unknown blob>".to_string()),
	}
}

/// No-op for readable value tag function
fn nop(_: &TagValue) -> String
{
	return "".to_string();
}

fn orientation(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Straight",
				3 => "Upside down",
				6 => "Rotated to left",
				8 => "Rotated to right",
				9 => "Undefined",
				_ => "(Invalid)",
			}
		},
		_ => "(Invalid)",
	};

	return s.to_string();
}

fn resolution_unit(e: &TagValue) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Unitless",
				2 => "in",
				3 => "cm",
				_ => "(Invalid)",
			}
		},
		_ => "(Invalid)",
	};

	return s.to_string();
}

/* Convert a numeric tag into EXIF tag and yiels info about the tag */
fn tag_to_exif(f: u16) -> (ExifTag, &'static str, &'static str, IfdFormat, u32, u32, fn(&TagValue) -> String)
{
	match f {

	0x010e =>
	(ExifTag::ImageDescription, "Image Description", "", IfdFormat::Str, -1, -1, nop),

	0x010f =>
	(ExifTag::Make, "Manufacturer", "", IfdFormat::Str, -1, -1, nop),

	0x0110 =>
	(ExifTag::Model, "Model", "", IfdFormat::Str, -1, -1, nop),

	0x0112 =>
	(ExifTag::Orientation, "Orientation", "", IfdFormat::U16, 1, 1, orientation),

	// TODO update unit with tag 0x0128
	0x011a =>
	(ExifTag::XResolution, "X Resolution", "@Resolution Unit", IfdFormat::URational, 1, 1, nop),

	// TODO update unit with tag 0x0128
	0x011b =>
	(ExifTag::YResolution, "Y Resolution", "@Resolution Unit", IfdFormat::URational, 1, 1, nop),

	0x0128 =>
	(ExifTag::ResolutionUnit, "Resolution Unit", "", IfdFormat::U16, 1, 1, resolution_unit),

	0x0131 =>
	(ExifTag::Software, "Software", "", IfdFormat::Str, -1, -1, nop),

	0x0132 =>
	(ExifTag::DateTime, "Image date", "", IfdFormat::Str, -1, -1, nop),

	0x013e =>
	(ExifTag::WhitePoint, "White Point", "CIE 1931 coordinates", IfdFormat::URational, 2, 2, nop),

	0x013f =>
	(ExifTag::PrimaryChromaticities, "Primary Chromaticities", "triple of CIE 1931 coordinates", IfdFormat::URational, 6, 6, nop),

	0x0211 =>
	(ExifTag::YCbCrCoefficients, "YCbCr Coefficients", "", IfdFormat::URational, 3, 3, nop),

	0x0213 =>
	(ExifTag::YCbCrPositioning, "YCbCr Positioning", "", IfdFormat::U16, 1, 1, nop),

	0x0214 =>
	(ExifTag::ReferenceBlackWhite, "Reference Black/White", "", IfdFormat::URational, 6, 6, nop),

	0x8298 =>
	(ExifTag::Copyright, "Copyright", "", IfdFormat::Str, -1, -1, nop),

	0x8769 =>
	(ExifTag::ExifOffset, "This image has an Exif SubIFD", "", IfdFormat::U32, 1, 1, nop),

	0x8825 =>
	(ExifTag::GPSOffset, "This image has a GPS SubIFD", "", IfdFormat::U32, 1, 1, nop),

	// FIXME check if it is reciprocal
	0x829a => (ExifTag::ExposureTime, "Exposure time", "s", IfdFormat::URational, 1, 1, nop),

	// FIXME check value
	0x829d => (ExifTag::FNumber, "f-number", "f-number", IfdFormat::URational, 1, 1, nop),

	// FIXME '1' means manual control, '2' program normal, '3' aperture priority, '4' shutter priority, '5' program creative (slow program), '6' program action(high-speed program), '7' portrait mode, '8' landscape mode.
	0x8822 => (ExifTag::ExposureProgram, "Exposure program", "", IfdFormat::U16, 1, 1, nop),

	0x8824 => (ExifTag::SpectralSensitivity, "Spectral sensitivity", "", IfdFormat::Str, -1, -1, nop),

	// FIXME 
	0x8827 => (ExifTag::ISOSpeedRatings, "ISO speed ratings", "ISO", IfdFormat::U16, 1, 2, nop),

	0x8828 => (ExifTag::OECF, "OECF", "", IfdFormat::Undefined, -1, -1, nop),

	0x9000 => (ExifTag::ExifVersion, "Exif version", "", IfdFormat::Undefined, -1, -1, nop),

	0x9003 => (ExifTag::DateTimeOriginal, "Date of original image", "", IfdFormat::Str, -1, -1, nop),

	0x9004 => (ExifTag::DateTimeDigitized, "Date of image digitalization", "", IfdFormat::Str, -1, -1, nop),

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
	0x9207 => (ExifTag::MeteringMode, "Meteting mode", "", IfdFormat::U16, 1, 1, nop),

	// FIXME http://www.awaresystems.be/imaging/tiff/tifftags/privateifd/exif/lightsource.html
	0x9208 => (ExifTag::LightSource, "Light source", "", IfdFormat::U16, 1, 1, nop),

	// FIXME
	0x9209 => (ExifTag::Flash, "Flash", "", IfdFormat::U16, 1, 2, nop),

	0x920a => (ExifTag::FocalLength, "Focal length", "mm", IfdFormat::URational, 1, 1, nop),

	// FIXME 
	0x9214 => (ExifTag::SubjectArea, "Subject area", "", IfdFormat::U16, 2, 4, nop),

	0x927c => (ExifTag::MakerNote, "Maker note", "", IfdFormat::Undefined, -1, -1, nop),

	0x9286 => (ExifTag::UserComment, "User comment", "", IfdFormat::Undefined, -1, -1, nop),

	0xa000 => (ExifTag::FlashPixVersion, "Flashpix version", "", IfdFormat::Undefined, -1, -1, nop),

	// FIXME
	0xa001 => (ExifTag::ColorSpace, "", "", IfdFormat::U16, 1, 1, nop),

	0xa004 => (ExifTag::RelatedSoundFile, "Related sound file", "", IfdFormat::Str, -1, -1, nop),

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
	0xa300 => (ExifTag::FileSource, "File source", "", IfdFormat::Undefined, 1, 1, nop),

	// FIXME
	0xa301 => (ExifTag::SceneType, "Scene type", "", IfdFormat::Undefined, 1, 1, nop),

	// FIXME
	0xa302 => (ExifTag::CFAPattern, "CFA Pattern", "", IfdFormat::Undefined, -1, -1, nop),

	// FIXME
	0xa401 => (ExifTag::CustomRendered, "", "", IfdFormat::U16, 1, 1, nop),

	0xa402 => (ExifTag::ExposureMode,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa403 => (ExifTag::WhiteBalanceMode,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa404 => (ExifTag::DigitalZoomRatio,
		 "", "", IfdFormat::URational, 1, 1, nop),

	0xa405 => (ExifTag::FocalLengthIn35mmFilm,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa406 => (ExifTag::SceneCaptureType,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa407 => (ExifTag::GainControl,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa408 => (ExifTag::Contrast,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa409 => (ExifTag::Saturation,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa40a => (ExifTag::Sharpness,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa40b => (ExifTag::DeviceSettingDescription,
		 "", "", IfdFormat::Undefined, -1, -1, nop),

	// FIXME
	0xa40c => (ExifTag::SubjectDistanceRange,
		 "", "", IfdFormat::U16, 1, 1, nop),

	0xa420 => (ExifTag::ImageUniqueID, "Image Unique ID", "", IfdFormat::Str, -1, -1, nop),
		
	0x0 => (ExifTag::GPSVersionID,
		 "", "", IfdFormat::U8, 4, 4, nop),

	// FIXME interpret
	0x1 => (ExifTag::GPSLatitudeRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	// FIXME and join with 0x1
	0x2 => (ExifTag::GPSLatitude,
		 "", "latitude deg.", IfdFormat::URational, 3, 3, nop),

	// FIXME interpret
	0x3 => (ExifTag::GPSLongitudeRef,
		 "", "longitude deg.", IfdFormat::Str, -1, -1, nop),

	// FIXME and join with 0x3
	0x4 => (ExifTag::GPSLongitude,
		 "", "degrees", IfdFormat::URational, 3, 3, nop),

	// FIXME
	0x5 => (ExifTag::GPSAltitudeRef,
		 "", "", IfdFormat::U8, 1, 1, nop),

	// FIXME
	0x6 => (ExifTag::GPSAltitude,
		 "GPS altitude", "m", IfdFormat::URational, 1, 1, nop),

	// FIXME
	0x7 => (ExifTag::GPSTimeStamp,
		 "GPS timestamp", "UTC", IfdFormat::URational, 3, 3, nop),

	0x8 => (ExifTag::GPSSatellites, "GPS satellites", "", IfdFormat::Str, -1, -1, nop),

	// FIXME interpret
	0x9 => (ExifTag::GPSStatus,
		 "GPS status", "", IfdFormat::Str, -1, -1, nop),

	// FIXME interpret
	0xa => (ExifTag::GPSMeasureMode,
		 "", "", IfdFormat::Str, -1, -1, nop),

	0xb => (ExifTag::GPSDOP,
		 "GPS Data Degree of Precision (DOP)", "deg.", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0xc => (ExifTag::GPSSpeedRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	// FIXME join with 0xc, show value
	0xd => (ExifTag::GPSSpeed,
		 "GPS speed", "", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0xe => (ExifTag::GPSTrackRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	0xf => (ExifTag::GPSTrack,
		 "GPS track", "deg.", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0x10 => (ExifTag::GPSImgDirectionRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	0x11 => (ExifTag::GPSImgDirection,
		 "", "", IfdFormat::URational, 1, 1, nop),

	0x12 => (ExifTag::GPSMapDatum, "GPS map datum", "", IfdFormat::Str, -1, -1, nop),

	// FIXME interpret
	0x13 => (ExifTag::GPSDestLatitudeRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	// FIXME
	0x14 => (ExifTag::GPSDestLatitude,
		 "", "", IfdFormat::URational, 3, 3, nop),

	// FIXME interpret
	0x15 => (ExifTag::GPSDestLongitudeRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	// FIXME
	0x16 => (ExifTag::GPSDestLongitude,
		 "", "", IfdFormat::URational, 3, 3, nop),

	// FIXME interpret
	0x17 => (ExifTag::GPSDestBearingRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	// FIXME
	0x18 => (ExifTag::GPSDestBearing,
		 "", "", IfdFormat::URational, 1, 1, nop),

	// FIXME interpret
	0x19 => (ExifTag::GPSDestDistanceRef,
		 "", "", IfdFormat::Str, -1, -1, nop),

	// FIXME
	0x1a => (ExifTag::GPSDestDistance,
		 "", "", IfdFormat::URational, 1, 1, nop),

	0x1b => (ExifTag::GPSProcessingMethod,
		 "", "", IfdFormat::Undefined, -1, -1, nop),

	0x1c => (ExifTag::GPSAreaInformation,
		 "", "", IfdFormat::Undefined, -1, -1, nop),

	0x1d => (ExifTag::GPSDateStamp,
		 "GPS date stamp", "", IfdFormat::Str, -1, -1, nop),

	// FIXME
	0x1e => (ExifTag::GPSDifferential,
		 "GPS differential", "", IfdFormat::U16, 1, 1, nop),
// EPX
	_ =>
	(ExifTag::UnknownToMe, "Unknown to this library, or manufacturer-specific", "Unknown unit",
		IfdFormat::Unknown, -1, -1, nop)
	}
}

/* Parse of raw IFD entry into EXIF data, if it is of a known type */
fn parse_exif_entry(f: &IfdEntry) -> ExifEntry
{
	let (value, readable_value) = tag_value(f);

	let mut e = ExifEntry {
			ifd: f.clone(),
			tag: ExifTag::UnknownToMe,
			value: value,
			unit: "Unknown".to_string(),
			tag_readable: format!("Unparsed tag {:x}", f.tag).to_string(),
			value_readable: readable_value.clone(),
			value_more_readable: readable_value,
			};

	let (tag, tag_readable, unit, format, min_count, max_count, more_readable) = tag_to_exif(f.tag);

	if tag == ExifTag::UnknownToMe {
		// Unknown EXIF tag type
		return e;
	}

	if (tag as u16) != f.tag ||
		(min_count == -1 && (format != IfdFormat::Str &&
				format != IfdFormat::Undefined &&
				format != IfdFormat::Unknown)) {
		panic!("Internal error {:x}", f.tag);
	}

	if format != f.format {
		writeln!(std::io::stderr(), "EXIF tag {:x} {}, expected format {}, found {}",
			f.tag, tag_readable, format as u8, f.format as u8);
		return e;
	}

	if min_count != -1 && (f.count < min_count || f.count > max_count) {
		writeln!(std::io::stderr(), "EXIF tag {:x} {}, format {}, expected count {}..{} found {}",
			f.tag, tag_readable, format as u8, min_count, max_count, f.count);
		return e;
	}

	e.tag = tag;
	e.tag_readable = tag_readable.to_string();
	e.unit = unit.to_string();

	if (more_readable as *const fn(&ExifTag) -> String) != (nop as *const fn(&ExifTag) -> String) {
		e.value_more_readable = more_readable(&e.value);
	}

	return e;
}


/* Superficial parse of IFD that can't fail */
fn parse_ifd(subifd: bool, le: bool, count: u16, contents: &[u8]) -> (Vec<IfdEntry>, usize)
{
	let mut entries: Vec<IfdEntry> = Vec::new();

	for i in 0..count {
		// println!("Parsing IFD entry {}", i);
		let mut offset = (i as usize) * 12;
		let tag = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let format = read_u16(le, &contents[offset..offset + 2]);
		offset += 2;
		let count = read_u32(le, &contents[offset..offset + 4]);
		offset += 4;
		let data = &contents[offset..offset + 4];
		let data = data.to_vec();

		let entry = IfdEntry{tag: tag, format: to_ifdformat(format), count: count,
					ifd_data: data, le: le,
					ext_data: Vec::new(), data: Vec::new()};
		entries.push(entry);
	}

	let next_ifd = match subifd {
		true => 0,
		false => read_u32(le, &contents[count as usize * 12..]) as usize
	};

	return (entries, next_ifd);
}

/* Deep parse of IFD that grabs EXIF data from IFD0 or SubIFD */
fn parse_exif_ifd(le: bool, contents: &[u8], ioffset: usize,
				exif_entries: &mut Vec<ExifEntry>) -> InExifResult
{
	let mut offset = ioffset;

	// println!("Offset is {}", offset);
	if contents.len() < (offset + 2) {
		return Err(ExifError{
			kind: ExifErrorKind::ExifIfdTruncated,
			extra: "Truncated at dir entry count".to_string()});
	}

	let count = read_u16(le, &contents[offset..offset + 2]);
	// println!("IFD entry count is {}", count);
	let ifd_length = (count as usize) * 12;
	offset += 2;

	if contents.len() < (offset + ifd_length) {
		return Err(ExifError{
			kind: ExifErrorKind::ExifIfdTruncated,
			extra: "Truncated at dir listing".to_string()});
	}

	let (mut ifd, _) = parse_ifd(true, le, count, &contents[offset..offset + ifd_length]);

	for entry in &mut ifd {
		entry.copy_data(&contents);
		let exif_entry = parse_exif_entry(&entry);
		exif_entries.push(exif_entry);
	}

	return Ok(());
}

/* Parses IFD0 and looks for SubIFD within IFD0 */
fn parse_ifds(le: bool, ifd0_offset: usize, contents: &[u8]) -> ExifResult
{
	let mut offset = ifd0_offset;
	let mut exif_entries: Vec<ExifEntry> = Vec::new();

	// fills exif_entries with data from IFD0

	match parse_exif_ifd(le, &contents, offset, &mut exif_entries) {
		Ok(_) => true,
		Err(e) => return Err(e),
	};

	// at this point we knot that IFD0 is good
	// looks for SubIFD (EXIF)

	let count = read_u16(le, &contents[offset..offset + 2]);
	let ifd_length = (count as usize) * 12 + 4;
	offset += 2;

	let (ifd, _) = parse_ifd(false, le, count, &contents[offset..offset + ifd_length]);

	for entry in &ifd {
		if entry.tag != (ExifTag::ExifOffset as u16) &&
				entry.tag != (ExifTag::GPSOffset as u16) {
			continue;
		}

		let exif_offset = entry.data_as_offset();

		if contents.len() < exif_offset {
			return Err(ExifError{
				kind: ExifErrorKind::ExifIfdTruncated,
				extra: "Exif SubIFD goes past EOF".to_string()});
		}

		match parse_exif_ifd(le, &contents, exif_offset, &mut exif_entries) {
			Ok(_) => true,
			Err(e) => return Err(e),
		};
	}

	return Ok(RefCell::new(ExifData{file: "".to_string(),
				size: 0,
				mime: "".to_string(),
				entries: exif_entries}));
}

/* Parse a TIFF image, or embedded TIFF in JPEG, in order to get IFDs and then the EXIF data */
pub fn parse_tiff(contents: &[u8]) -> ExifResult
{
	let mut le = false;

	if contents.len() < 8 {
		return Err(ExifError{
			kind: ExifErrorKind::TiffTruncated,
			extra: "".to_string()});
	} else if contents[0] == ('I' as u8) &&
			contents[1] == ('I' as u8) &&
			contents[2] == 42 && contents[3] == 0 {
		/* TIFF little-endian */
		le = true;
	} else if contents[0] == ('M' as u8) && contents[1] == ('M' as u8) &&
			contents[2] == 0 && contents[3] == 42 {
		/* TIFF big-endian */
	} else {
		let err = format!("Preamble is {:x} {:x} {:x} {:x}",
			contents[0], contents[1],
			contents[2], contents[3]);
		return Err(ExifError{
			kind: ExifErrorKind::TiffBadPreamble,
			extra: err.to_string()});
	}

	let offset = read_u32(le, &contents[4..8]) as usize;

	return parse_ifds(le, offset, &contents);
}

/* Parse an image buffer that may be of any format. Detect format and find EXIF data */
pub fn parse_buffer(fname: &str, contents: &Vec<u8>) -> ExifResult
{
	let mime = detect_type(&contents);

	if mime == "" {
		return Err(ExifError{
				kind: ExifErrorKind::FileTypeUnknown,
				extra: "".to_string()});
	}

	let mut offset = 0 as usize;
	let mut size = contents.len() as usize;

	if mime == "image/jpeg" {
		let (eoffset, esize, err) = find_embedded_tiff_in_jpeg(&contents);
		if eoffset == 0 {
			return Err(ExifError{
				kind: ExifErrorKind::JpegWithoutExif,
				extra: err.to_string()});
		}
		offset = eoffset;
		size = esize;
		// println!("Offset {} size {}", offset, size);
	}
	match parse_tiff(&contents[offset .. offset + size]) {
		Ok(d) => {
				d.borrow_mut().size = contents.len();
				d.borrow_mut().file = fname.to_string();
				d.borrow_mut().mime = mime.to_string();
				Ok(d)
			},
		Err(e) => Err(e)
	}

}

/* Read and interpret an image file */
pub fn read_file(fname: &str, f: &mut File) -> ExifResult
{
	match f.seek(SeekFrom::Start(0)) {
		Ok(_) => (),
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileSeekError,
				extra: fname.to_string()}),
	}

	// TODO: should read only the relevant parts of a file,
	// and pass a StringIO-like object instead of a Vec buffer

	let mut contents: Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Ok(_) => parse_buffer(&fname, &contents),
		Err(_) => Err(ExifError{kind: ExifErrorKind::FileReadError,
				extra: fname.to_string()}),
	}
}

/* Parse an image file */
pub fn parse_file(fname: &str) -> ExifResult
{
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(_) => return Err(ExifError{kind: ExifErrorKind::FileOpenError,
				extra: fname.to_string()}),
	};
	return read_file(fname, &mut f);
}

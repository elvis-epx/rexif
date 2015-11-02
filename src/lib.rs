use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
use std::result::Result;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::cell::RefCell;
use std::cell::Cell;

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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum ExifTag {
	Unrecognized,
	ImageDescription,
	Make,
	Model,
	Orientation,
	XResolution,
	YResolution,
	ResolutionUnit,
	Software,
	DateTime,
	WhitePoint,
	PrimaryChromaticities,
	YCbCrCoefficients,
	YCbCrPositioning,
	ReferenceBlackWhite,
	Copyright,
	ExposureTime,
	FNumber,
	ExposureProgram,
	ISOSpeedRatings,
	ExifVersion,
	DateTimeOriginal,
	DateTimeDigitized,
	ComponentConfiguration,
	CompressedBitsPerPixel,
	ShutterSpeedValue,
	ApertureValue,
	BrightnessValue,
	ExposureBiasValue,
	MaxApertureValue,
	SubjectDistance,
	MeteringMode,
	LightSource,
	Flash,
	FocalLength,
	MakerNote,
	UserComment,
	FlashPixVersion,
	ColorSpace,
	ExifImageWidth,
	ExifImageHeight,
	RelatedSoundFile,
	FocalPlaneXResolution,
	FocalPlaneYResolution,
	FocalPlaneResolutionUnit,
	SensingMethod,
	FileSource,
	SceneType,
}

#[derive(Clone)]
pub struct ExifEntry {
	ifd: IfdEntry,
	tag: ExifTag,
	format: IfdFormat,
	value: TagValue,
	unit: String,
	tag_readable: String,
	data_readable: String,
}

#[derive(Copy, Clone)]
pub struct URational {
	numerator: u32,
	denominator: u32,
	value: f64,
}

#[derive(Copy, Clone)]
pub struct IRational {
	numerator: i32,
	denominator: i32,
	value: f64,
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
			IfdFormat::Unknown => 0,
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
			self.data.clear();
			self.data.extend(&self.ifd_data[..]);
			return true;
		}

		let offset = self.data_as_offset();
		if contents.len() < (offset + self.length()) {
			println!("EXIF data block goes beyond EOF");
			return false;
		}

		let ext_data = &contents[offset..offset + self.length()];
		self.ext_data.clear();	
		self.ext_data.extend(ext_data);
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

/* Read a u16 from a stream of bytes */
fn read_u16(le: bool, raw: &[u8]) -> u16
{
	if le {
		(raw[1] as u16) * 256 + raw[0] as u16
	} else {
		(raw[0] as u16) * 256 + raw[1] as u16
	}
}

/* Read a u32 from a stream of bytes */
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

/* Parse of raw IFD entry into EXIF data, if it is of a known type */
fn parse_exif_entry(f: &IfdEntry) -> ExifEntry
{
	let mut e = ExifEntry{
			ifd: f.clone(),
			tag: ExifTag::Unrecognized,
			tag_readable: "Unrecognized".to_string(),
			format: IfdFormat::Unknown,
			value: TagValue::Unknown(f.data.clone()),
			unit: "Unknown".to_string(),
			data_readable: "".to_string(),
			};

	// FIXME

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
		println!("Reading EXIF tag {:x}", entry.tag);
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
		// println!("Reading tag {:x}", entry.tag);
		if entry.tag != 0x8769 {
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

		break;
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

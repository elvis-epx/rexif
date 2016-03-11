use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::error::Error;
use super::types::*;
use super::lowlevel::*;

/// Convert an IFD format code to the IfdFormat enumeration
pub fn ifdformat_new(n: u16) -> IfdFormat
{
	match n {
		1 => IfdFormat::U8,
		2 => IfdFormat::Ascii,
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
	/// Casts IFD entry data into an offset. Not very useful for the crate client.
	/// The call can't fail, but the caller must be sure that the IFD entry uses
	/// the IFD data area as an offset (i.e. when the tag is a Sub-IFD tag, or when
	/// there are more than 4 bytes of data and it would not fit within IFD).
	pub fn data_as_offset(&self) -> usize {
		read_u32(self.le, &(self.ifd_data[0..4])) as usize
	}

	/// Returns the size of an individual element (e.g. U8=1, U16=2...). Every
	/// IFD entry contains an array of elements, so this is NOT the size of the
	/// whole entry!
	pub fn size(&self) -> u8
	{
		match self.format {
			IfdFormat::U8 => 1,
			IfdFormat::Ascii => 1,
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

	/// Total length of the whole IFD entry (element count x element size)
	pub fn length(&self) -> usize
	{
		(self.size() as usize) * (self.count as usize)
	}

	/// Returns true if data is contained within the IFD structure, false when
	/// data can be found elsewhere in the image (and IFD structure contains the
	/// data offset, instead of data).
	pub fn in_ifd(&self) -> bool
	{
		self.length() <= 4
	}

	/// Copies data from IFD entry section reserved for data (up to 4 bytes), or
	/// from another part of the image file (when data wouldn't fit in IFD structure).
	/// In either case, the data member will contain the data of interest after
	/// this call.
	pub fn copy_data(&mut self, contents: &[u8]) -> bool
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
	/// Human-readable string with the meaning of the error enumeration
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

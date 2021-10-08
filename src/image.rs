use crate::chunk::{Chunk, ChunkError};

use std::convert::TryInto;


#[derive(Debug)]
pub enum ColorType {
	Unknown,
	Greyscale,
	Truecolor,
	IndexedColor,
	GreyscaleAlpha,
	TruecolorAlpha
}

impl From<isize> for ColorType {
	fn from(num: isize) -> Self {
		match num {
			0 => Self::Greyscale,
			2 => Self::Truecolor,
			3 => Self::IndexedColor,
			4 => Self::GreyscaleAlpha,
			6 => Self::TruecolorAlpha,
			_ => Self::Unknown
		}
	}
}


#[derive(Debug)]
pub struct ImageInfo<'a> {
	pub filepath: &'a str,

	pub width: u32,
	pub height: u32,
	pub bit_depth: u8,
	pub color_type: ColorType,
	pub compression_method: u8,
	pub filter_method: u8,
	pub interlace_method: u8
}

impl<'a> ImageInfo<'a> {
	pub fn empty() -> Self {
		ImageInfo {
			filepath: "",
			width: 0,
			height: 0,
			bit_depth: 0,
			color_type: ColorType::Unknown,
			compression_method: 0,
			filter_method: 0,
			interlace_method: 0
		}
	}

	pub fn from_info(filepath: &'a str, ihdr_chunk: &Chunk) -> Result<Self, ChunkError> {
		let mut info = Self::from_ihdr(ihdr_chunk)?;
		info.filepath = filepath;

		Ok(info)
	}

	pub fn from_ihdr(chunk: &Chunk) -> Result<Self, ChunkError> {
		if !is_header_chunk(chunk) {
			return Err(ChunkError::new("Chunk type is not IHDR".into()));
		}

		Ok(ImageInfo {
			filepath: "",
			width: u32::from_be_bytes(chunk.data[0..4].try_into().unwrap()),
			height: u32::from_be_bytes(chunk.data[4..8].try_into().unwrap()),
			bit_depth: chunk.data[8],
			color_type: (chunk.data[9] as isize).into(),
			compression_method: chunk.data[10],
			filter_method: chunk.data[11],
			interlace_method: chunk.data[12]
		})
	}
}

pub fn is_header_chunk(chunk: &Chunk) -> bool {
	&chunk.ch_type == b"IHDR"
}

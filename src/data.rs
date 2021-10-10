// Provide structs & functions for image data collection / manipulation

use crate::chunk::*;
use crate::image::ImageInfo;

use flate2::read::ZlibDecoder;

use std::fs::File;
use std::io::{BufWriter, Read};


pub trait Pixel {
	fn write_to(&self, stream: &mut DataStream);
}


pub struct ImageData<'a> {
	pub info: ImageInfo<'a>,
	pixels: Vec<u8>
}

impl<'a> ImageData<'a> {
	pub fn empty() -> Self {
		ImageData {
			info: ImageInfo::empty(),
			pixels: vec![]
		}
	}

	pub fn from_info(info: ImageInfo<'a>) -> Self {
		ImageData {
			info,
			pixels: vec![]
		}
	}


	pub fn add_info(&mut self, info: ImageInfo<'a>) {
		self.info = info;
	}

	pub fn add_data(&mut self, data_chunk: &Chunk) -> Result<(), ChunkError> {
		if !is_data_chunk(data_chunk) {
			return Err(ChunkError::new("Not a data chunk".into()));
		}

		self.decode_data(data_chunk)
	}


	// Dedode data and add it to the pixel array
	fn decode_data(&mut self, data_chunk: &Chunk) -> Result<(), ChunkError> {
		let mut decoder = ZlibDecoder::new(data_chunk.data.as_slice());
		let mut decoded = Vec::new();

		match decoder.read_to_end(&mut decoded) {
			Ok(_) => { self.pixels.append(&mut decoded); },
			Err(e) => return Err(ChunkError::new(format!("Couldn't decompress pixel data: {}", e).into()))
		}

		Ok(())
	}
}

pub fn is_data_chunk(chunk: &Chunk) -> bool {
	&chunk.ch_type == b"IDAT"
}


pub struct DataStream {
	pub encode_info: Chunk,
	stream: BufWriter<std::fs::File>
}

impl DataStream {
	pub fn open(filepath: &str) -> Result<Self, std::io::Error> {
		let file = File::open(filepath)?;

		Ok(DataStream {
			encode_info: Chunk::empty(),
			stream: BufWriter::new(file)
		})
	}

	pub fn write(&mut self, _data: &[u8]) {
		// TODO
	}
}

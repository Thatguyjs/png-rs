// Provide structs & functions for image data collection / manipulation

use crate::chunk::*;

use std::fs::File;
use std::io::BufWriter;


pub trait Pixel {
	fn write_to(&self, stream: &mut DataStream);
}


pub struct ImageData<T: Pixel> {
	pub data_info: Chunk,
	pixels: Vec<T>
}

impl<T: Pixel> ImageData<T> {
	pub fn empty() -> Self {
		ImageData {
			data_info: Chunk::empty(),
			pixels: vec![]
		}
	}

	pub fn from_info(data_info: Chunk) -> Self {
		ImageData {
			data_info,
			pixels: vec![]
		}
	}


	pub fn add_info(&mut self, info_chunk: Chunk) {
		self.data_info = info_chunk;
	}

	pub fn add_data(&mut self, data_chunk: &Chunk) -> Result<(), ChunkError> {
		if !is_data_chunk(data_chunk) {
			ChunkError::new("Not a data chunk".into());
		}

		// TODO: Add chunk data

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

	pub fn write(&mut self, data: &[u8]) {
		// TODO
	}
}

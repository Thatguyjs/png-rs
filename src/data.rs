// Provide structs & functions for image data collection / manipulation

use crate::chunk::*;
use crate::image::ImageInfo;

use std::fs::File;
use std::io::BufWriter;


pub trait Pixel {
	fn write_to(&self, stream: &mut DataStream);
}


pub struct ImageData<'a, T: Pixel> {
	pub info: ImageInfo<'a>,
	pixels: Vec<T>
}

impl<'a, T: Pixel> ImageData<'a, T> {
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

	pub fn write(&mut self, _data: &[u8]) {
		// TODO
	}
}

use crate::chunk::Chunk;

use std::io::{BufWriter, Write};
use std::fs::File;


pub struct ImageEncoder<'a> {
	pub filepath: &'a str,
	buffer: BufWriter<File>
}

impl<'a> ImageEncoder<'a> {
	pub fn open(filepath: &'a str) -> Result<Self, std::io::Error> {
		let file = File::create(filepath)?;

		let mut buffer = BufWriter::new(file);
		Self::write_sig(&mut buffer)?;

		Ok(ImageEncoder {
			filepath,
			buffer
		})
	}

	fn write_sig(buffer: &mut BufWriter<File>) -> Result<usize, std::io::Error> {
		buffer.write(&[137, 80, 78, 71, 13, 10, 26, 10])
	}

	pub fn write_chunk(&mut self, chunk: &Chunk) -> Result<(), std::io::Error> {
		chunk.write_to(&mut self.buffer)
	}
}


#[derive(Debug)]
pub struct EncoderError {
	pub message: String
}

impl EncoderError {
	pub fn new(message: String) -> Self {
		EncoderError { message }
	}
}

impl std::fmt::Display for EncoderError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str(&self.message)
	}
}

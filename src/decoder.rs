#[path="./chunk.rs"]
mod chunk;
use chunk::{Chunk, Chunks, ChunkError};

use std::fs::File;
use std::io::{BufReader, Read};


pub struct ImageDecoder<'a> {
	pub filepath: &'a str,
	buffer: BufReader<File>
}

impl<'a> ImageDecoder<'a> {
	pub fn open(filepath: &'a str) -> Result<Self, DecoderError> {
		let file = File::open(filepath);

		match file {
			Ok(f) => {
				let mut buffer = BufReader::new(f);
				
				match ImageDecoder::check_sig(&mut buffer) {
					Ok(_) => Ok(ImageDecoder { filepath, buffer }),
					Err(e) => Err(DecoderError::new( e.to_string()))
				}
			},
			Err(e) => Err(DecoderError::new(e.to_string()))
		}
	}

	fn check_sig(buffer: &mut BufReader<File>) -> Result<(), DecoderError> {
		let mut sig_buf = [0u8; 8];
		let expected: &[u8; 8] = &[137, 80, 78, 71, 13, 10, 26, 10];

		match buffer.read_exact(&mut sig_buf) {
			Ok(_) => {
				for (n1, n2) in sig_buf.iter().zip(expected.iter()) {
					if n1 != n2 { return Err(DecoderError::new("Invalid image signature".into())) }
				}

				Ok(())
			},
			Err(e) => Err(DecoderError::new(e.to_string()))
		}
	}

	pub fn next_chunk(&mut self) -> Result<Chunk, ChunkError> {
		Chunk::next_from_buffer(&mut self.buffer)
	}

	pub fn chunks(&mut self) -> Chunks {
		Chunks::new(&mut self.buffer)
	}
}


#[derive(Debug)]
pub struct DecoderError {
	pub message: String
}

impl DecoderError {
	pub fn new(message: String) -> Self {
		DecoderError { message }
	}
}

impl std::fmt::Display for DecoderError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str(&self.message)
	}
}

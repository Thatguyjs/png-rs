// Read chunks from an image stream

use std::io::{BufReader, Read};
use std::fs::File;


pub struct ChunkError {
	message: String
}

impl ChunkError {
	pub fn new(message: String) -> Self {
		ChunkError { message }
	}
}

impl std::fmt::Display for ChunkError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str(&self.message)
	}
}


pub struct Chunk {
	pub length: u32,
	pub ch_type: [u8; 4],
	pub data: Vec<u8>,
	pub crc: [u8; 4]
}

impl Chunk {
	// Construct an "empty" chunk with no meaningful data
	fn empty() -> Self {
		Chunk {
			length: 0,
			ch_type: [0; 4],
			data: vec![],
			crc: [0; 4]
		}
	}

	pub fn parse(stream: &mut BufReader<File>) -> Result<Self, ChunkError> {
		let mut chunk = Chunk::empty();
		let mut length_buf = [0u8; 4];

		match stream.read_exact(&mut length_buf) {
			Ok(_) => { chunk.length = u32::from_be_bytes(length_buf); },
			Err(_) => return Err(ChunkError::new("Couldn't read chunk length".into()))
		}

		match stream.read_exact(&mut chunk.ch_type) {
			Ok(_) => {},
			Err(_) => return Err(ChunkError::new("Couldn't read chunk type".into()))
		}

		if chunk.length > 0 {
			chunk.data = vec![0u8; chunk.length as usize];

			match stream.read_exact(&mut chunk.data) {
				Ok(_) => {},
				Err(_) => return Err(ChunkError::new("Couldn't read chunk data".into()))
			}
		}

		match stream.read_exact(&mut chunk.crc) {
			Ok(_) => {},
			Err(_) => return Err(ChunkError::new("Couldn't read chunk CRC".into()))
		}

		Ok(chunk)
	}
}

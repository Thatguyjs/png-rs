use std::io::{BufReader, Read};
use std::fs::File;


#[derive(Debug, PartialEq, Clone)]
pub enum ChunkLocation {
	Start,
	End,
	Unknown
}


#[derive(Debug, Clone)]
pub struct ChunkProperties {
	pub ancillary: bool,
	pub private: bool,
	pub reserved: bool,
	pub safe_to_copy: bool
}

impl ChunkProperties {
	pub fn new() -> Self {
		ChunkProperties {
			ancillary: true,
			private: false,
			reserved: false,
			safe_to_copy: false
		}
	}

	pub fn parse_type(ch_type: &[u8; 4], props: &mut ChunkProperties) {
		props.ancillary = (ch_type[0] & 32) != 0;
		props.private = (ch_type[1] & 32) != 0;
		props.reserved = (ch_type[2] & 32) != 0;
		props.safe_to_copy = (ch_type[3] & 32) != 0;
	}
}


#[derive(Debug, Clone)]
pub struct Chunk {
	pub length: u32,
	pub ch_type: [u8; 4],
	pub properties: ChunkProperties,
	pub data: Vec<u8>,
	pub crc: [u8; 4],
	pub location: ChunkLocation
}

impl Chunk {
	pub fn new(length: u32, ch_type: [u8; 4], data: Vec<u8>, crc: [u8; 4]) -> Self {
		Chunk {
			length,
			ch_type,
			properties: ChunkProperties::new(),
			data,
			crc,
			location: Chunk::get_location(&ch_type)
		}
	}

	pub fn empty() -> Self {
		Chunk {
			length: 0,
			ch_type: [0; 4],
			properties: ChunkProperties::new(),
			data: vec![],
			crc: [0; 4],
			location: ChunkLocation::Unknown
		}
	}

	pub fn get_location(ch_type: &[u8; 4]) -> ChunkLocation {
		if ch_type == b"IHDR" { return ChunkLocation::Start; }
		else if ch_type == b"IEND" { return ChunkLocation::End; }
		ChunkLocation::Unknown
	}

	pub fn next_from_buffer(buffer: &mut BufReader<File>) -> Result<Chunk, ChunkError> {
		let mut chunk = Chunk::empty();
		let mut length_buf = [0u8; 4];

		match buffer.read_exact(&mut length_buf) {
			Ok(_) => { chunk.length = u32::from_be_bytes(length_buf); },
			Err(e) => return Err(ChunkError::new(format!("Failed to read chunk length: {}", e)))
		}

		match buffer.read_exact(&mut chunk.ch_type) {
			Ok(_) => { ChunkProperties::parse_type(&chunk.ch_type, &mut chunk.properties); },
			Err(e) => return Err(ChunkError::new(format!("Failed to read chunk type: {}", e)))
		}

		if chunk.length > 0 {
			chunk.data = vec![0; chunk.length as usize];

			match buffer.read_exact(&mut chunk.data) {
				Ok(_) => {},
				Err(e) => return Err(ChunkError::new(format!("Failed to read chunk data: {}", e)))
			}
		}

		match buffer.read_exact(&mut chunk.crc) {
			Ok(_) => {},
			Err(e) => return Err(ChunkError::new(format!("Failed to read chunk CRC: {}", e)))
		}

		chunk.location = Chunk::get_location(&chunk.ch_type);
		Ok(chunk)
	}
}


pub struct Chunks<'a> {
	buffer: &'a mut BufReader<File>,
	is_end: bool
}

impl<'a> Chunks<'a> {
	pub fn new(buffer: &'a mut BufReader<File>) -> Self {
		Chunks { buffer, is_end: false }
	}
}

impl<'a> Iterator for Chunks<'a> {
	type Item = Result<Chunk, ChunkError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.is_end { return None; }

		match Chunk::next_from_buffer(self.buffer) {
			Ok(chunk) => {
				if Chunk::get_location(&chunk.ch_type) == ChunkLocation::End {
					self.is_end = true;
				}

				Some(Ok(chunk))
			},
			Err(e) => {
				self.is_end = true;
				Some(Err(e))
			}
		}
	}
}


#[derive(Debug)]
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

#![allow(dead_code)]

mod chunk;
use chunk::{Chunk, ChunkError};

use flate2::read::ZlibDecoder;

use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;
use std::convert::TryInto;


pub struct ImageError {
	message: String
}

impl ImageError {
	pub fn new(message: String) -> Self {
		ImageError { message }
	}
}

impl std::fmt::Display for ImageError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "An image error occured")
	}
}


pub struct ImageInfo {
	pub width: u32,
	pub height: u32,
	pub bit_depth: u8,
	pub color_type: u8,
	pub compression_method: u8,
	pub filter_method: u8,
	pub interlace_method: u8
}

impl ImageInfo {
	pub fn empty() -> Self {
		ImageInfo {
			width: 0,
			height: 0,
			bit_depth: 0,
			color_type: 0,
			compression_method: 0,
			filter_method: 0,
			interlace_method: 0
		}
	}
}

impl std::fmt::Debug for ImageInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,  "ImageInfo {{ \
						width: {}, \
						height: {}, \
						bit_depth: {}, \
						color_type: {}, \
						compression_method: {}, \
						filter_method: {}, \
						interlace_method: {} \
					}}",
			self.width,
			self.height,
			self.bit_depth,
			self.color_type,
			self.compression_method,
			self.filter_method,
			self.interlace_method
		)
	}
}


pub struct ImageTime {
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8
}

impl ImageTime {
	pub fn empty() -> Self {
		ImageTime {
			year: 0,
			month: 0,
			day: 0,
			hour: 0,
			minute: 0,
			second: 0
		}
	}
}

impl std::fmt::Debug for ImageTime {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,  "ImageTime {{ \
						year: {}, \
						month: {}, \
						day: {}, \
						hour: {}, \
						minute: {}, \
						second: {} \
					}}",
			self.year,
			self.month,
			self.day,
			self.hour,
			self.minute,
			self.second
		)
	}
}


pub struct ImageData {
	info: ImageInfo,
	data: Vec<u8>,

	index: usize
}

impl ImageData {
	pub fn new(info: ImageInfo) -> Self {
		ImageData {
			info,
			data: vec![],
			index: 0
		}
	}

	pub fn load_data(&mut self, data: Vec<u8>) {
		self.data = data;
		println!("Got Data:\n{:?}", self.data);
	}
}

impl Iterator for ImageData {
	type Item = (u8, u8, u8, u8);

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}


pub struct Image<'a> {
	pub filepath: &'a str,
	stream: BufReader<File>,
	pub chunks: Vec<Chunk>,

	pub info: ImageInfo,
	pub time: ImageTime,
	pub keywords: HashMap<String, String>,

	pub data: ImageData
}

impl<'a> Image<'a> {
	pub fn open(filepath: &'a str) -> Result<Self, std::io::Error> {
		let file = File::open(filepath)?;

		Ok(Image {
			filepath,
			stream: BufReader::new(file),
			chunks: vec![],

			info: ImageInfo::empty(),
			time: ImageTime::empty(),
			keywords: HashMap::new(),

			data: ImageData::new(ImageInfo::empty())
		})
	}

	// Read and verify the image's signature
	pub fn read_sig(&mut self) -> Result<(), ImageError> {
		let mut sig_buf: [u8; 8] = [0; 8];
		let expected: &[u8; 8] = &[137, 80, 78, 71, 13, 10, 26, 10];

		match self.stream.read_exact(&mut sig_buf) {
			Ok(_) => {
				for (n1, n2) in sig_buf.iter().zip(expected.iter()) {
					if n1 != n2 { return Err(ImageError::new("Invalid image signature".into())); }
				}

				Ok(())
			},
			Err(_) => Err(ImageError::new("Couldn't read the image signature".into()))
		}
	}

	// Parse chunk data and add data to self if needed
	fn parse_chunk(&mut self, chunk: &Chunk) -> Result<(), ChunkError> {
		if &chunk.ch_type == b"IHDR" {
			// TODO: Make these catch errors
			self.info.width = u32::from_be_bytes(chunk.data[0..4].try_into().unwrap());
			self.info.height = u32::from_be_bytes(chunk.data[4..8].try_into().unwrap());
			self.info.bit_depth = chunk.data[8];
			self.info.color_type = chunk.data[9];
			self.info.compression_method = chunk.data[10];
			self.info.filter_method = chunk.data[11];
			self.info.interlace_method = chunk.data[12];
		}

		else if &chunk.ch_type == b"tIME" {
			// TODO: Catch errors
			self.time.year = u16::from_be_bytes(chunk.data[0..2].try_into().unwrap());
			self.time.month = chunk.data[2];
			self.time.day = chunk.data[3];
			self.time.hour = chunk.data[4];
			self.time.minute = chunk.data[5];
			self.time.second = chunk.data[6];
		}

		else if &chunk.ch_type == b"tEXt" {
			let mut strings =
				chunk.data.split(|byte| byte == &0)
				.map(|string| std::str::from_utf8(string));

			let (key, value): (String, String);

			match strings.next() {
				Some(val) => {
					match val {
						Ok(k) => { key = k.into(); }
						Err(_) => return Err(ChunkError::new("Couldn't read the key of a tEXt chunk".into()))
					}
				},
				None => return Err(ChunkError::new("Couldn't read the key of a tEXt chunk".into()))
			}

			match strings.next() {
				Some(val) => {
					match val {
						Ok(v) => { value = v.into(); }
						Err(_) => return Err(ChunkError::new("Couldn't read the value of a tEXt chunk".into()))
					}
				},
				None => return Err(ChunkError::new("Couldn't read the value of a tEXt chunk".into()))
			}

			self.keywords.insert(key, value);
		}

		else if &chunk.ch_type == b"IDAT" {
			let mut decoder = ZlibDecoder::new(chunk.data.as_slice());
			let mut decoded = Vec::new();

			match decoder.read_to_end(&mut decoded) {
				Ok(_) => { self.data.load_data(decoded); },
				Err(e) => return Err(ChunkError::new(format!("Couldn't decompress pixel data: {}", e).into()))
			}
		}

		Ok(())
	}

	// Read a single chunk from the image
	pub fn read_chunk(&mut self) -> Result<&Chunk, ChunkError> {
		match Chunk::parse(&mut self.stream) {
			Ok(chunk) => {
				match self.parse_chunk(&chunk) {
					Ok(_) => {},
					Err(e) => return Err(e)
				}

				self.chunks.push(chunk);
				Ok(self.chunks.last().unwrap()) // The previous push() should garuntee that there's at least 1 item here
			},
			Err(ce) => Err(ce)
		}
	}
}

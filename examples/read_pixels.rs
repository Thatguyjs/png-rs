use png_rs::{decoder::*, data::*};


struct RGB(u8, u8, u8);

impl png_rs::data::Pixel for RGB {
	fn write_to(&self, stream: &mut DataStream) {
		stream.write(&[self.0, self.1, self.2]);
	}
}


fn main() -> Result<(), DecoderError> {
	let mut decoder = ImageDecoder::open("tests/sample.png")?;
	let mut data = ImageData::<RGB>::empty(); // TODO: Rename `empty()` methods to `default()` and maybe implement the `Default` trait

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => {
				if is_data_chunk(c) {
					println!("Data chunk: {:?}", c);
					data.add_data(c).unwrap();
				}
			},
			Err(e) => { println!("Got error: {}", e); }
		}
	}

	Ok(())
}

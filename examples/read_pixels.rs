use png_rs::{decoder::*, data::{ImageData, is_data_chunk}};


struct RGB(u8, u8, u8);

impl png_rs::data::Pixel for RGB {
	type Dest = std::fs::File;

	fn write_to(buffer: &mut std::io::BufWriter<Self::Dest>) {

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

use png_rs::{decoder::*, encoder::*};


fn main() -> Result<(), DecoderError> {
	let mut decoder = ImageDecoder::open("tests/sample.png")?;
	let mut encoder = ImageEncoder::open("tests/sample_copy.png").expect("Failed to create ImageEncoder");

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => {
				encoder.write_chunk(c).expect("Failed to write chunk");
				println!("Wrote chunk");
			},
			Err(e) => { println!("Got error: {}", e); }
		}
	}

	Ok(())
}

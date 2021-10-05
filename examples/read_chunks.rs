#[path="../src/lib.rs"]
mod png;
use png::decoder::*;


fn main() -> Result<(), DecoderError> {
	let mut decoder = ImageDecoder::open("tests/sample.png")?;

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => { println!("Got chunk: {:?}", c); },
			Err(e) => { println!("Got error: {}", e); }
		}
	}

	Ok(())
}

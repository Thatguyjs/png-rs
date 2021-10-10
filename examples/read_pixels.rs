use png_rs::{data::*, decoder::*, image::*};


fn main() -> Result<(), DecoderError> {
	let mut decoder = ImageDecoder::open("tests/sample.png")?;
	let mut data = ImageData::empty(); // TODO: Rename `empty()` methods to `default()` and maybe implement the `Default` trait

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => {
				if is_header_chunk(c) {
					let info = ImageInfo::from_info("tests/sample.png", c).unwrap();

					println!("{:?}", info);
					data.add_info(info);
				}

				if is_data_chunk(c) {
					data.add_data(c).unwrap();
				}
			},
			Err(e) => { println!("Got error: {}", e); }
		}
	}

	Ok(())
}

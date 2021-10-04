#[path="../src/lib.rs"]
mod png;
use png::*;


fn main() -> Result<(), std::io::Error> {
	let mut image = Image::open("tests/sample.png")?;
	
	match image.read_sig() {
		Ok(_) => {},
		Err(e) => { panic!("Error reading image signature: {}", e); }
	}

	loop {
		match image.read_chunk() {
			Ok(c) => {
				println!("Length: {}\nType: {}\nData: {:?}\nCRC: {:?}\n", c.length, std::str::from_utf8(&c.ch_type).unwrap(), &c.data, c.crc);

				// Ending chunk
				if &c.ch_type == b"IEND" {
					break;
				}
			},
			Err(e) => {
				println!("Error reading chunk: {}", e);
				break;
			}
		}
	}

	println!("Info: {:?}", image.info);
	println!("Time: {:?}", image.time);
	println!("Keywords: {:?}", image.keywords);
	Ok(())
}

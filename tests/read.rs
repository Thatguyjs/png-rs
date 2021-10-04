#[path="../src/lib.rs"]
mod png;
use png::*;



#[test]
fn open_image() {
	let image = Image::open("tests/sample.png");

	match image {
		Ok(_) => {},
		Err(e) => { panic!("Opening image failed: {:?}", e); }
	}
}

#[test]
fn read_sig() -> Result<(), std::io::Error> {
	let mut image = Image::open("tests/sample.png")?;
	
	match image.read_sig() {
		Ok(_) => Ok(()),
		Err(e) => { panic!("Error reading image signature: {}", e); }
	}
}

#[test]
fn read_chunk() -> Result<(), std::io::Error> {
	let mut image = Image::open("tests/sample.png")?;
	
	match image.read_sig() {
		Ok(_) => {},
		Err(e) => { panic!("Error reading image signature: {}", e); }
	}

	match image.read_chunk() {
		Ok(_) => {},
		Err(e) => { panic!("Error reading image chunk: {}", e); }
	}

	Ok(())
}

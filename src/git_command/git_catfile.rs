use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf; 
use flate2::read::ZlibDecoder;

pub fn git_catfile(filehash: &str) -> Result<(), String>{

	let (dir, file) = filehash.split_at(2);
	let mut target_path = PathBuf::new();
	target_path.push("./.subgit/objects");
	target_path.push(dir);
	target_path.push(file);

	let mut buf = Vec::new();
	let mut f = match File::open(target_path){
		Ok(file) => file,
		Err(e) => return Err(e.to_string()),
	};
	match f.read_to_end(&mut buf){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	}

	let mut decoder = ZlibDecoder::new(&buf[..]);
	let mut content: Vec<u8> = Vec::new();
	match decoder.read_to_end(&mut content){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};

	let mut original = content.splitn(2, |&x| x == b'\0');
	println!("<header>\n {}", String::from_utf8(original.next().unwrap().to_vec()).unwrap());
	println!("<content>\n {}", String::from_utf8(original.next().unwrap().to_vec()).unwrap());


	return Ok(());
}
use std::io::{Read, Write};
use std::{path::PathBuf};
use std::fs::{File, self};
use flate2::Compression;
use sha1::{Digest, Sha1};
use flate2::write::ZlibEncoder;

pub fn git_hashobj(filepath: &String) -> Result<(), String>{

	let mut path = PathBuf::new();
	path.push("./");

  path.push(PathBuf::from(filepath));
	let mut f = match File::open(path){
		Ok(file) => file,
    Err(e) => return Err(e.to_string()),
	};

  let mut buf = String::new(); 
  match f.read_to_string(&mut buf){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};
  
	let content = format!("blob {}\0{}", buf.len(), buf);  
	let blob_hash = Sha1::digest(content.as_bytes());

	println!("hash: {:x}",blob_hash);
	let hash_str = format!("{:x}",blob_hash);

	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
	match encoder.write_all(&content.as_bytes()){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};

	let compressed = match encoder.finish(){
		Ok(comp) => comp,
		Err(e) => return Err(e.to_string()),
	};

	
	let (dir, file) = hash_str.split_at(2);
	
	let mut save_path = PathBuf::new();
	save_path.push("./.subgit/objects");
	save_path.push(dir);

	let save_dir = save_path.clone();
	match fs::create_dir_all(save_dir){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	}

	save_path.push(file);
	
	let mut f = match File::create(save_path){
		Ok(file) => file,
		Err(e) => return Err(e.to_string()),
	};

	match f.write_all(&compressed){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};

	match f.flush(){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};
	return Ok(());
}
use std::{path::PathBuf, fs::File, os::unix::prelude::MetadataExt, io::{Read, Write}};

use sha1::{Sha1, Digest};

pub fn padding(name_size: usize) -> usize {
	
	let size_ctime_to_namesize = 38;

  let remainder = (name_size + size_ctime_to_namesize) % 8;
  let padding = 8 - remainder;
  return padding;
}

fn create_entry(name: &str) -> Result<Vec<u8>, String> {

	let mut path = PathBuf::new();
	path.push("./");
	path.push(name);

	let metadata = match path.metadata(){
		Ok(meta) => meta,
		Err(e) => return Err(e.to_string()),
	};

	let ctime = metadata.ctime() as u32;
	let mtime = metadata.mtime() as u32;
	let mode = metadata.mode() as u32;
	let size = metadata.size() as u32;

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
	let hash = blob_hash.as_slice();
	
	let name_size = name.len() as u16;
	let padding_size = padding(name_size as usize);

	let meta = [
		ctime.to_be_bytes(), //4bytes 0-3
		mtime.to_be_bytes(), //4bytes 4-7
		mode.to_be_bytes(), //4bytes 8-11
		size.to_be_bytes(), //4bytes 12-15
	].concat();

	let entry_meta = [
		meta,
		hash.to_vec(), //20bytes 16-35
		Vec::from(name_size.to_be_bytes()), //2bytes 36-37 ここまでで38bytes
		name.as_bytes().to_vec(), //?bytes
		vec![b'\0'; padding_size], 
	].concat();

	return Ok(entry_meta);
}



pub fn write_index(filename: Vec<&str>) -> Result<(), String> {

	let mut index:Vec<Vec<u8>>= vec![];

	let dirc = b"DIRC"; //4bytes
	let version = 2 as u32; //4bytes
	let entry_num = filename.len() as u32; //4bytes
	let header = [*dirc, version.to_be_bytes(), entry_num.to_be_bytes()].concat();
	index.push(header);

	for name in filename {
    let entry = create_entry(name)?;
    index.push(entry)
  }

	let mut path = PathBuf::new();
  path.push("./.subgit/index");

  let mut file = match File::create(path){
		Ok(f) => f,
		Err(e) => return Err(e.to_string()),
	};

  match file.write_all(index.concat().as_slice()){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};
  match file.flush(){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	}

	return Ok(());
}
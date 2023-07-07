use std::{path::PathBuf, fs::{File, self}, io::Write};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Sha1, Digest};

use crate::index::read_index::read_index;

pub fn git_write_tree() -> Result<String, String> {
	let entries = match read_index(){
		Ok(entry_list) => entry_list,
    Err(e) => return Err(e.to_string()),
	};

	let mut entry_list = vec![];

	for entry in entries {
		let entry_header = format!("{:0>6o} {}\0", entry.mode, String::from_utf8(entry.filename).ok().unwrap());
		let entry_content = [entry_header.as_bytes(), &entry.hash].concat();
		entry_list.push(entry_content);
	}

  let header_tree = format!("tree {}\0", entry_list.concat().len());
  let content_tree = [header_tree.as_bytes().to_vec(), entry_list.concat()].concat();
	let tree_hash = Sha1::digest(&content_tree);

	let hash_str = format!("{:x}", tree_hash);

	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  match encoder.write_all(&content_tree){
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
	
	return Ok(hash_str);
}
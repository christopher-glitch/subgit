use std::{io::Write, path::PathBuf, fs::{File, self}};

use flate2::{write::ZlibEncoder, Compression};
use sha1::{Sha1, Digest};

use crate::commit::{read_commit::read_commit, read_refs::read_refs};


pub fn git_commit_tree(tree_hash: &str, message: &str) -> Result<Option<String>, String> {
	let tree_hash = format!("tree {}", tree_hash);
	
	let commit_content = match read_refs()? {
		Some(h) => {
			if h.len() == 0 {
				let content = format!("{}\n\n{}\n",tree_hash, message);
				format!("commit {}\0{}", content.len(), content).as_bytes().to_vec()
			}
			else if tree_hash.as_str() != read_commit(h.as_str())? {
				let parent = format!("parent {}", h);
				let content = format!("{}\n{}\n\n{}\n",tree_hash, parent, message);
				format!("commit {}\0{}", content.len(), content).as_bytes().to_vec()
			}
			else {
				return Ok(None);
			}
		}
		_ => {
      let content = format!("{}\n\n{}\n", tree_hash, message);
      format!("commit {}\0{}", content.len(), content).as_bytes().to_vec()
    }
	};

	let commit_hash = Sha1::digest(&commit_content);
	let hash_str = format!("{:x}", commit_hash);

	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  match encoder.write_all(&commit_content){
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
	
	return Ok(Some(hash_str));
}
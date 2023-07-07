use std::{path::PathBuf, fs::File, io::Read};

pub fn read_refs() -> Result<Option<String>, String> {

    let mut path = PathBuf::new();
    path.push(PathBuf::from(".subgit/refs/main")); //mainブランチのみ

    let mut file = match File::open(path){
			Ok(f) => f,
			Err(e) => return Err(e.to_string()),
		};

    let mut hash = String::new();
    match file.read_to_string(&mut hash){
			Ok(_) => {},
			Err(e) => return Err(e.to_string()),
		};

    return Ok(Some(hash.replace("\n", "")));
}
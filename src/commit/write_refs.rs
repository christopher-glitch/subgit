use std::{path::PathBuf, fs::File, io::Write};

pub fn write_refs(hash: String) -> Result<(), String> {

    let mut path = PathBuf::new();
    path.push(PathBuf::from(".subgit/refs/main")); //mainブランチのみ

    let mut file = match File::create(path){
			Ok(f) => f,
			Err(e) => return Err(e.to_string()),
		};
    
    match file.write_all(hash.as_bytes()){
      Ok(_) => {},
      Err(e) => return Err(e.to_string()),
    };
    match file.flush(){
      Ok(_) => {},
      Err(e) => return Err(e.to_string()),
    };

    return Ok(());
}
use std::fs;
use std::fs::File;

pub fn git_init() -> Result<(), String>{
	match fs::create_dir("./.subgit") {
        Ok(_) => {},
        Err(_) => {
            return Err("failed to create gitc dir".to_string());
        }
    }

	match File::create("./.subgit/index") {
        Ok(_) => {},
        Err(_) => {
            return Err("failed to create index file".to_string());
        }
    }

	match fs::create_dir("./.subgit/refs") {
        Ok(_) => {},
        Err(_) => {
            return Err("failed to create refs dir".to_string());
        }
    }

	match File::create("./.subgit/refs/main") {
        Ok(_) => {},
        Err(_) => {
            return Err("failed to create main file".to_string());
        }
    }

	match fs::create_dir("./.subgit/objects") {
        Ok(_) => {},
        Err(_) => {
            return Err("failed to create objects dir".to_string());
        }
    }
	
	return Ok(());
}
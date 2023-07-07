use crate::{index::read_index::read_index};

pub fn git_ls_files() -> Result<(), String> {
	let entries = match read_index(){
		Ok(entry_list) => entry_list,
    Err(e) => return Err(e.to_string()),
	};

	if entries.len() == 0 {
		println!("There is no staging file");
		return Ok(());
	}

	for entry in entries {
		println!("mode: {:0>6o}, hash: {}, filename: {}", entry.mode,  hex::encode(entry.hash), String::from_utf8(entry.filename).ok().unwrap());
	}
	return Ok(());
}
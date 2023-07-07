use crate::{index::{write_index::write_index, read_index::read_index}};

pub fn git_update_index(filepath: &str) -> Result<(), String>{
	let entries = match read_index(){
		Ok(entry_list) => entry_list,
		Err(e) => return Err(e.to_string()),
	};
	
	//indexに何もない場合
	if entries.len() == 0 {
		match write_index(vec![filepath]){
			Ok(_) => {},
			Err(e) => return Err(e.to_string()),
		};
	//indexに書き込まれている場合
	}else {
		//indexの中身を全て取り出す
		let mut paths: Vec<String> = vec![];

		for entry in entries {
			let filename = String::from_utf8(entry.filename).ok().unwrap();
			paths.push(filename);
		}

		//indexの中身に被っているものがないか確認
		if !paths.iter().any(|p| p==&filepath) {
			paths.push(filepath.to_owned())
		}
		
		let paths_str = paths.iter().map(|s| &**s).collect();

		match write_index(paths_str){
			Ok(_) => {},
			Err(e) => return Err(e.to_string()),
		}
	}

	return Ok(());
}
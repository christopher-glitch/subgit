use std::{path::PathBuf, fs::File, io::Read};
use byteorder::{BigEndian, ByteOrder};

use super::{util::write_index::{write_index, padding}, parameter::index::{ENTRY_NUM, FNSIZE_START, FNSIZE_END, FNAME_START}};



pub fn git_update_index(filepath: &str) -> Result<(), String>{
	let mut path = PathBuf::new();
	path.push("./.subgit/index");

	//indexを読む
	let buf = match File::open(path) {
		Ok(mut f) => {
			let mut buf: Vec<u8> = vec![];
			match f.read_to_end(&mut buf){
				Ok(_) => {},
				Err(e) => return Err(e.to_string()),
			};
			buf
		},
		Err(_) => {
			vec![]
		}
	};
	
	//indexに何もない場合
	if buf == vec![] {
		match write_index(vec![filepath]){
			Ok(_) => {},
			Err(e) => return Err(e.to_string()),
		};
	//indexに書き込まれている場合
	}else {
		//indexの中身を全て取り出す
		let mut paths: Vec<String> = vec![];
		let entry_num = BigEndian::read_u32(&buf[ENTRY_NUM..(ENTRY_NUM+4)]) as usize;
		let mut start = 12 as usize;


		for _i in 0..entry_num {
			let filename_size = BigEndian::read_u16(&buf[(start+FNSIZE_START)..(start+FNSIZE_END)]) as u16;
			let filename = (&buf[(start+FNAME_START)..(start+FNAME_START+filename_size as usize)]).to_vec();
			let padding = padding(filename_size as usize);
			start = start + FNAME_START + filename_size as usize + padding;

			let filename = String::from_utf8(filename).ok().unwrap();
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
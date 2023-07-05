use std::{path::PathBuf, fs::File, io::Read};

use byteorder::{BigEndian, ByteOrder};

use super::{parameter::index::{ENTRY_NUM, FNSIZE_START, FNAME_START, FNSIZE_END, MODE_START, MODE_END, HASH_START, HASH_END}, util::write_index::padding};

pub fn git_ls_files() -> Result<(), String> {
	let mut path = PathBuf::new();
	path.push("./.subgit/index");

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

	let entry_num = BigEndian::read_u32(&buf[ENTRY_NUM..(ENTRY_NUM+4)]) as usize;
	let mut start = 12 as usize;

	for _i in 0..entry_num {

		let mode = BigEndian::read_u32(&buf[(start+MODE_START)..(start+MODE_END)]) as u32;

    let hash = (&buf[(start+HASH_START)..(start+HASH_END)]).to_vec();

		let filename_size = BigEndian::read_u16(&buf[(start+FNSIZE_START)..(start+FNSIZE_END)]) as u16;

		let filename = (&buf[(start+FNAME_START)..(start+FNAME_START+filename_size as usize)]).to_vec();

		let padding = padding(filename_size as usize);

		start = start + FNAME_START + filename_size as usize + padding;

		println!("mode: {:0>6o}, hash: {}, filename: {}", mode,  hex::encode(hash), String::from_utf8(filename).ok().unwrap())
	}
	return Ok(());
}
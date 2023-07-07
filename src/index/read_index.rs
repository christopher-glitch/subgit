use std::{path::PathBuf, fs::File, io::Read};

use byteorder::{BigEndian, ByteOrder};

use crate::{index::index_macro::{MODE_START, MODE_END, HASH_START, HASH_END, FNSIZE_START, FNSIZE_END, FNAME_START}, util::padding::padding};

use super::index_macro::ENTRY_NUM;

pub struct IndexContent {
	pub mode: u32,
	pub hash: Vec<u8>,
	pub filename_size: u16,
	pub filename: Vec<u8>,
}


pub fn read_index() -> Result<Vec<IndexContent>, String> {
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

	if buf == vec![] {
		return Ok(vec![]);
	}

	let entry_num = BigEndian::read_u32(&buf[ENTRY_NUM..(ENTRY_NUM+4)]) as usize;
	let mut start = 12 as usize;

	let mut entries: Vec<IndexContent> = Vec::new();

	for _i in 0..entry_num {
		let mode = BigEndian::read_u32(&buf[(start+MODE_START)..(start+MODE_END)]) as u32;
    let hash = (&buf[(start+HASH_START)..(start+HASH_END)]).to_vec();
		let filename_size = BigEndian::read_u16(&buf[(start+FNSIZE_START)..(start+FNSIZE_END)]) as u16;
		let filename = (&buf[(start+FNAME_START)..(start+FNAME_START+filename_size as usize)]).to_vec();

		let padding = padding(filename_size as usize);
		start = start + FNAME_START + filename_size as usize + padding;

		let idcontent = IndexContent{mode, hash, filename_size, filename};
		entries.push(idcontent);
	}
	return Ok(entries);
}
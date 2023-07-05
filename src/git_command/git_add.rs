use super::{git_hashobj::git_hashobj, git_updateid::git_update_index};

pub fn git_add(filepath: &String) -> Result<(), String> {
	match git_hashobj(filepath){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};
  match git_update_index(filepath){
		Ok(_) => {},
		Err(e) => return Err(e.to_string()),
	};

  return Ok(());
}
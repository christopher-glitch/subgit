use crate::commit::write_refs::write_refs;

pub fn git_update_ref(commit_hash: String) -> Result<(), String>{
	match write_refs(commit_hash){
		Ok(_) => return Ok(()),
		Err(e) => return Err(e),
	}
}
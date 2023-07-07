use super::{git_commit_tree::git_commit_tree, git_updateref::git_update_ref, git_write_tree::git_write_tree};

pub fn git_commit(message: &str) -> Result<(), String> {
    let tree_hash = match git_write_tree(){
			Ok(h) => h,
			Err(e) => return Err(e.to_string()),
		};

    match git_commit_tree(&tree_hash, message)? {
        Some(c) => match git_update_ref(c){
					Ok(_) => {},
					Err(e) => return Err(e.to_string()),
				},
        _ => println!("Nothing to commit"),
    };

    return Ok(());
}
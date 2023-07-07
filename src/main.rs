use std::{env::{self}, path::{Path}};
use subgit::git_command;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        eprintln!("args Error: input commmand");
        return Err(());
    }

    if args[1] == "init" {
        match git_command::git_init::git_init() {
            Ok(_) => println!("subgit_init complete"),
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "hash-object" {
        if !Path::new("./.subgit").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 3 {
            eprintln!("Error: please input a file path.");
            return Err(());
        }

        let filepath = &args[2];
        match git_command::git_hashobj::git_hashobj(filepath) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "cat-file" {
        if !Path::new("./.subgit").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 3 {
            eprintln!("Error: please input a blob hash code.");
            return Err(());
        }

        let filehash = &args[2];
        match git_command::git_catfile::git_catfile(filehash) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "update-index" {
        if !Path::new("./.subgit/index").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 3 {
            eprintln!("Error: please input a file path.");
            return Err(());
        }

        let filepath = &args[2];
        match git_command::git_updateid::git_update_index(filepath) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "ls-files" {
        if !Path::new("./.subgit/index").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        match git_command::git_lsfile::git_ls_files() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "add" {
        if !Path::new("./.subgit/index").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 3 {
            eprintln!("Error: please input a file path.");
            return Err(());
        }

        let filepath = &args[2];
        match git_command::git_add::git_add(filepath) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "write-tree" {
        if !Path::new("./.subgit").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        match git_command::git_write_tree::git_write_tree() {
            Ok(tree_hash) => {
                println!("tree hash: {}", tree_hash);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        }
    }

    if args[1] == "commit-tree" {
        if !Path::new("./.subgit").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 4 {
            eprintln!("Error: please input a tree hash code and message");
            return Err(());
        }

        let tree_hash = &args[2];
        let message = &args[3];
        let commit_hash = match git_command::git_commit_tree::git_commit_tree(tree_hash, message) {
            Ok(hash) => hash,
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        };

        match commit_hash {
            Some(h) => {
                println!("commit hash: {}", h);
            }
            None => {
                println!("Nothing to commit");
            },
        }
    }

    if args[1] == "update-refs" {
        if !Path::new("./.subgit").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 3 {
            eprintln!("Error: please input a commit hash code");
            return Err(());
        }

        let commit_hash = &args[2];
        match git_command::git_updateref::git_update_ref(commit_hash.to_string()) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        };
    }

    if args[1] == "commit" {
        if !Path::new("./.subgit").exists() {
            eprintln!("Error: please run subgit init.");
            return Err(());
        }

        if args.len() != 3 {
            eprintln!("Error: please input a message");
            return Err(());
        }

        let message = &args[2];
        match git_command::git_commit::git_commit(message) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(());
            }
        };
    }

    return Ok(());
}

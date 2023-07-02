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

    return Ok(());
}

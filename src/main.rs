use std::env::{self};
use subgit::git_command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("Args Error: input commmand");
        return;
    }

    if args[1] == "init" {
        match git_command::git_init::git_init() {
            Ok(_) => println!("gitc_init complete"),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    if args[1] == "add" {

    }


    println!("Hello, world!");
}

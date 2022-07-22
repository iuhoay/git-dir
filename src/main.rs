#![allow(unused)]

use clap::Parser;
use git2::Repository;
use std::{ffi::OsString, fs::DirEntry, path::Path};

#[macro_use]
extern crate colour;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, parse(from_os_str), default_value = ".")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;

    match std::fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                display_dir(path.unwrap());
            }
        }
    }
}

fn display_dir(path: DirEntry) -> () {
    if path.file_type().unwrap().is_dir() {
        let repo = match Repository::open(path.path()) {
            Ok(repo) => display_branch(path.file_name().to_str().unwrap(), repo),
            Err(_) => return,
        };
    }
}

fn display_branch(path_name: &str, repo: Repository) -> () {
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return,
    };
    let head_name = head.shorthand().unwrap();
    print!("{} - ", path_name);
    blue_ln!("[{}]", head_name);
}

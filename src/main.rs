extern crate git2;

use std::env;
use git2::*;

fn main() {
    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => panic!("failed to read current directory: {}", e),
    };
    println!("Current dir is {}", current_dir.display());

    //let current_dir: std::path::PathBuf = [r"/home", "josea", "my_repos", "git_cmd"].iter().collect();
    //println!("Current dir is {}", current_dir.display());

    let my_repo = match Repository::init(current_dir) {
        Ok(my_repo) => my_repo,
        Err(e) => panic!("Faile:d to open: {}", e),
    };

    let my_head = match my_repo.head() {
        Ok(my_head) => my_head,
        Err(e) => panic!("Can't read HEAD: {}", e),
    };

    println!("Hello Wold");

    //let name = match Reference::normalize_name(my_head, ReferenceFormat::NORMAL) {
    //    Ok(name) => name,
    //    Err(e) => panic!("Can't normalize name"),
    //};
    //println!("{}", name);
}

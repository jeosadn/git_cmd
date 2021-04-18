extern crate git2;

use git2::{Repository, Branch};
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Invalid dir");

    let repo = Repository::init(current_dir).expect("Invalid repo");

    let branch_name: String;

    if repo.head_detached().expect("Invalid HEAD") {
        branch_name = "detached".to_string();
    } else {
        let head = repo.head().expect("Invalid HEAD");
        let branch: Branch = Branch::wrap(head);
        branch_name = branch
            .name()
            .expect("Invalid branch")
            .expect("Invalid branch")
            .to_string();
    }
    print!("({})", branch_name);
}

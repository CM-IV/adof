use crate::git::{is_remote_exist, remote::link_remote};

pub fn link(repo_link: &str) {
    if is_remote_exist() {
        println!("Already exists");
        std::process::exit(1);
    }

    link_remote(repo_link);
}

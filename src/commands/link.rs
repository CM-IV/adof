use crate::git::{remote::link_remote, is_remote_exist};

pub fn link(repo_link: &str) {
    if is_remote_exist() {
        println!("Already exists");
        std::process::exit(1);
    }
    
    link_remote(repo_link);
}

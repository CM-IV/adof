use crate::git::{get_repo, is_remote_exist};

pub fn unlink() {
    let repo = get_repo();

    if is_remote_exist() {
        repo.remote_delete("origin").unwrap();
    } else {
        println!("first connect to remote");
    }
}

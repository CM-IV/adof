use crate::git::{is_remote_exist, remote::unlink_remote};

pub fn unlink() {
    if is_remote_exist() {
        unlink_remote();
    } else {
        println!("first connect to remote");
    }
}

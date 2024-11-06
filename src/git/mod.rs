use chrono::Local;
use git2::{Repository, Signature};

use adof::get_adof_dir;

pub mod add;
pub mod commit;
pub mod commit_message;
pub mod git_ignore;

pub fn init_git() {
    let adof_dir = get_adof_dir();
    Repository::init(adof_dir).unwrap();
    add::git_add()
}

pub fn get_repo() -> Repository {
    let adof_dir = get_adof_dir();
    Repository::open(adof_dir).unwrap()
}

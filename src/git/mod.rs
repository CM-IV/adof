use chrono::Local;
use git2::Repository;

use adof::get_adof_dir;

pub mod add;
pub mod commit;
pub mod commit_message;
pub mod git_ignore;
pub mod local;
pub mod remote;

pub struct Commit {
    pub id: String,
    pub message: String,
}

impl Commit {
    fn new(hash: &str, message: &str) -> Self {
        Self {
            id: hash.to_string(),
            message: message.to_string(),
        }
    }
}

pub fn init_git() {
    let adof_dir = get_adof_dir();
    Repository::init(adof_dir).unwrap();
    add::git_add()
}

pub fn get_repo() -> Repository {
    let adof_dir = get_adof_dir();
    Repository::open(adof_dir).unwrap()
}

pub fn is_remote_exist() -> bool {
    get_repo().find_remote("origin").is_ok()
}

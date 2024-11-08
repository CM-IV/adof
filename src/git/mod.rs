use chrono::Local;
use git2::Repository;

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

pub fn is_remote_exist() -> bool {
    get_repo().find_remote("origin").is_ok()
}

pub fn get_default_branch() -> String {
    let repo = get_repo();
    let config = repo.config().unwrap();

    if let Ok(default_branch) = config.get_string("init.defaultBranch") {
        default_branch.to_string()
    } else {
        "master".to_string()
    }
}

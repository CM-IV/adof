use chrono::Local;
use git2::{Repository, Signature};

use adof::get_adof_dir;

pub mod add;
pub mod branch;
pub mod commit;
pub mod commit_message;
pub mod git_ignore;
pub mod merge;

pub fn init_git() {
    let adof_dir = get_adof_dir();
    Repository::init(adof_dir).unwrap();
    add::git_add()
}

pub fn get_repo() -> Repository {
    let adof_dir = get_adof_dir();
    Repository::open(adof_dir).unwrap()
}

fn get_signature() -> Signature<'static> {
    let repo = get_repo();
    let config = repo.config().unwrap();
    let name = config
        .get_string("user.name")
        .unwrap_or("Unknown".to_string());
    let email = config
        .get_string("user.email")
        .unwrap_or("unknown@example.com".to_string());

    Signature::now(&name, &email).unwrap()
}

fn get_default_branch() -> String {
    let repo = get_repo();

    let config = repo.config().unwrap();

    if let Ok(default_branch) = config.get_string("init.defaultBranch") {
        default_branch.to_string()
    } else {
        "master".to_string()
    }
}

fn get_today() -> String {
    let current_date_time = Local::now().naive_local();
    current_date_time.format("%e %b %Y").to_string()
}

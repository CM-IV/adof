use chrono::Local;
use git2::{BranchType, Repository, Signature};

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

fn get_old_branch() -> String {
    let repo = get_repo();
    let default_branch = get_default_branch();
    let mut old_branch = String::new();

    let branches_iter = repo.branches(Some(BranchType::Local)).unwrap();

    for branch in branches_iter {
        let (branch, _) = branch.unwrap();
        let branch_name = branch.name().unwrap().unwrap_or_default().to_string();

        if branch_name != default_branch {
            old_branch = branch_name;
        }
    }

    old_branch
}

fn get_today() -> String {
    let current_date_time = Local::now().naive_local();
    current_date_time
        .format("%M")
        .to_string()
        .trim()
        .to_string()
}

fn get_current_date_and_time() -> String {
    let current_time = Local::now().naive_local();
    let formatted_current_time = current_time.format("%a,%e %b %Y %l:%M %p");
    formatted_current_time.to_string()
}

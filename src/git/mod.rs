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

fn get_current_date_and_time() -> String {
    let current_time = Local::now().naive_local();
    let formatted_current_time = current_time.format("%a,%e %b %Y %l:%M %p");
    formatted_current_time.to_string()
}

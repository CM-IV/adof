use chrono::Local;
use git2::{BranchType, Signature};

use crate::git::commit_message::get_commit_message;

use super::*;

pub fn commit() {
    let commit_message = get_commit_message();

    if is_new_day() {
    } else {
        commit_changes(&commit_message);
    }
}

fn get_today() -> String {
    let current_date_time = Local::now().naive_local();
    current_date_time.format("%e %b %Y").to_string()
}

fn is_new_day() -> bool {
    let today = get_today();
    get_repo().find_branch(&today, BranchType::Local).is_err()
}

fn commit_changes(commit_message: &str) {
    let repo = get_repo();
    let mut index = repo.index().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let parent_commit = match repo.head() {
        Ok(head_ref) => Some(head_ref.peel_to_commit().unwrap()),
        Err(_) => None,
    };

    let config = repo.config().unwrap();
    let name = config
        .get_string("user.name")
        .unwrap_or("Unknown".to_string());
    let email = config
        .get_string("user.email")
        .unwrap_or("unknown@example.com".to_string());

    let signature = Signature::now(&name, &email).unwrap();

    if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent],
        )
        .unwrap()
    } else {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[],
        )
        .unwrap()
    };
}

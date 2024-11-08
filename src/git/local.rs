use std::process;

use super::*;

pub fn get_local_commits(num: u8) -> Vec<Commit> {
    if num != 0 {
        get_local_commit(num)
    } else if !is_remote_exist() {
        get_local_commit(5)
    } else {
        let only_local_changes = get_only_local_commits();

        if only_local_changes.is_empty() {
            get_local_commit(5)
        } else {
            only_local_changes
        }
    }
}

fn get_only_local_commits() {
    let default_branch = get_default_branch();
}

pub fn get_only_local_commits_no() -> usize {}

fn get_local_commit(num: u8) {}

fn get_remote_commit(num: u8) {}

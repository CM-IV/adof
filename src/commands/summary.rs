use std::path::Path;

use crate::database::get_table_struct;
use crate::git::{
    is_remote_exist,
    local::{get_local_commits, get_remote_behind_commits_no},
    remote::get_remote_commits,
    Commit,
};

use super::*;

pub fn summary() {
    let _ = get_last_local_commit();
    let _ = get_last_remote_commit();
    let _ = get_only_local_commits_no();
    let _ = get_files_being_tracked_no();
    let _ = is_auto_update_enabled();
}

fn get_last_local_commit() -> Vec<Commit> {
    get_local_commits(1)
}

fn get_last_remote_commit() -> Vec<Commit> {
    if !is_remote_exist() {
        println!("Connect to remote first");
    }

    get_remote_commits(1)
}

fn get_only_local_commits_no() -> usize {
    get_remote_behind_commits_no()
}

fn get_files_being_tracked_no() -> usize {
    let table_struct = get_table_struct();
    table_struct.table.len()
}

fn is_auto_update_enabled() -> bool {
    let pid_file = get_pid_file();
    Path::new(&pid_file).exists()
}

// fn check_for_update() -> Vec<String> {}

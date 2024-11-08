use std::process::Command;
use std::env;

use adof::get_adof_dir;

use crate::git::get_default_branch;

pub fn log(num: u8, remote: bool) {
    if remote {
        show_remote_commits(num);
    } else {
        if num == 0 {
            show_only_local_commits();
        } else {
            show_local_commits(num);
        }
    }
}

fn show_local_commits(num: u8) {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let default_branch = get_default_branch();

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("-n")
        .arg(&num.to_string())
        .arg(&default_branch)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
}

fn show_remote_commits(num: u8) {}

fn show_only_local_commits() {}

// fn get_only_local_commits_no() -> u8 {5}

use std::env;
use std::process::Command;

use adof::get_adof_dir;

use crate::git::{get_default_branch, is_remote_exist};

pub fn log(num: u8, remote: bool) {
    if remote && is_remote_exist() {
        show_remote_commits(num);
    } else if num == 0 && is_remote_exist() {
        get_only_local_commits_no(); // remove it from here, it is here just for avoid compiler
                                     // warning
        show_only_local_commits();
    } else {
        show_local_commits(num);
    }
}

fn show_local_commits(mut num: u8) {
    if num == 0 {
        num = 5
    }

    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let default_branch = get_default_branch();

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg("-n")
        .arg(num.to_string())
        .arg(default_branch)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
}

fn show_remote_commits(num: u8) {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let remote_branch = "origin/main";

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg("-n")
        .arg(num.to_string())
        .arg(remote_branch)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
}

fn show_only_local_commits() {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let default_branch = get_default_branch();
    let diff_branch = format!("origin/main..{}", default_branch);

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg(diff_branch)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
}

fn get_only_local_commits_no() -> u8 {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let default_branch = get_default_branch();
    let diff_branch = format!("origin/main..{}", default_branch);

    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(diff_branch)
        .output()
        .unwrap();

    let count_str = std::str::from_utf8(&output.stdout).unwrap().trim();
    println!("{:?}", count_str);
    count_str.parse::<u8>().unwrap()
}

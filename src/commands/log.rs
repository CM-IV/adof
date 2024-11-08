use std::env;
use std::process::Command;

use adof::get_adof_dir;

use crate::git::get_default_branch;

pub fn log(num: u8, remote: bool) {
    if remote {
        show_remote_commits(num);
    } else if num == 0 {
        get_only_local_commits_no();
        show_only_local_commits();
    } else {
        show_local_commits(num);
    }
}

fn highlight_git_log(log: &str) -> String {
    let mut highlighted = String::new();
    for line in log.lines() {
        if line.starts_with("commit ") {
            highlighted.push_str(&format!("\x1b[32m{}\x1b[0m\n", line)); // Green for commit hash
        } else if line.starts_with("Author: ") {
            highlighted.push_str(&format!("\x1b[34m{}\x1b[0m\n", line)); // Blue for author
        } else if line.starts_with("Date: ") {
            highlighted.push_str(&format!("\x1b[35m{}\x1b[0m\n", line)); // Magenta for date
        } else {
            highlighted.push_str(&format!("{}\n", line)); // No color for other lines
        }
    }
    highlighted
}

fn show_local_commits(num: u8) {
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
    let highlighted_log = highlight_git_log(stdout);
    println!("{}", highlighted_log);
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
    let highlighted_log = highlight_git_log(stdout);
    println!("{}", highlighted_log);
}

fn show_only_local_commits() {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let default_branch = get_default_branch();
    let diff_branch = format!("{}..{}", "origin/main", default_branch);

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg(diff_branch)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let highlighted_log = highlight_git_log(stdout);
    println!("{}", highlighted_log);
}

fn get_only_local_commits_no() -> u8 {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let default_branch = get_default_branch();
    let diff_branch = format!("{}..{}", "origin/main", default_branch);

    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(diff_branch)
        .output()
        .unwrap();

    let count_str = std::str::from_utf8(&output.stdout).unwrap().trim();
    count_str.parse::<u8>().unwrap()
}

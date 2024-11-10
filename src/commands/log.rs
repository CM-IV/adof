use std::env;
use std::process::Command;

use adof::get_adof_dir;

use crate::git::{get_default_branch, is_remote_exist};

use super::*;

pub fn log(num: u8, remote: bool) -> Result<()> {
    if remote && is_remote_exist() {
        show_remote_commits(num)?;
    } else if num == 0 && is_remote_exist() {
        if get_only_local_commits_no()? == 0 {
            println!("Everything is upto date.");
            show_local_commits(5)?;
        }
        show_only_local_commits()?;
    } else {
        show_local_commits(num)?;
    }

    Ok(())
}

fn show_local_commits(num: u8) -> Result<()> {
    let adof_dir = get_adof_dir()?;
    env::set_current_dir(adof_dir)?;

    let default_branch = get_default_branch()?;

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg("-n")
        .arg(num.to_string())
        .arg(default_branch)
        .output()?;

    let stdout = std::str::from_utf8(&output.stdout)?;
    println!("{}", stdout);
    Ok(())
}

fn show_remote_commits(mut num: u8) -> Result<()> {
    if num == 0 {
        num = 5;
    }

    let adof_dir = get_adof_dir()?;
    env::set_current_dir(adof_dir)?;

    let remote_branch = "origin/main";

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg("-n")
        .arg(num.to_string())
        .arg(remote_branch)
        .output()?;

    let stdout = std::str::from_utf8(&output.stdout)?;
    println!("{}", stdout);
    Ok(())
}

fn show_only_local_commits() -> Result<()> {
    let adof_dir = get_adof_dir()?;
    env::set_current_dir(adof_dir)?;

    let default_branch = get_default_branch()?;
    let diff_branch = format!("origin/main..{}", default_branch);

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg(diff_branch)
        .output()?;

    let stdout = std::str::from_utf8(&output.stdout)?;
    println!("{}", stdout);
    Ok(())
}

fn get_only_local_commits_no() -> Result<u8> {
    let adof_dir = get_adof_dir()?;
    env::set_current_dir(adof_dir)?;

    let default_branch = get_default_branch()?;
    let diff_branch = format!("origin/main..{}", default_branch);

    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(diff_branch)
        .output()?;

    let count_str = std::str::from_utf8(&output.stdout)?.trim();
    println!("{:?}", count_str);
    Ok(count_str.parse::<u8>().unwrap_or_default())
}

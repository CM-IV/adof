use std::env;
use std::process::Command;

use anyhow::{Context, Result};

use crate::git::{get_default_branch, is_remote_exist};
use adof::get_adof_dir;

pub fn log(num: u8, remote: bool) -> Result<()> {
    if remote && is_remote_exist().context("Failed to check if remote exists")? {
        show_remote_commits(num)?;
    } else if num == 0 && is_remote_exist().context("Failed to check if remote exists")? {
        if get_only_local_commits_no()? == 0 {
            println!("Everything is up to date.");
            show_local_commits(5)?;
        }
        show_only_local_commits()?;
    } else {
        show_local_commits(num)?;
    }
    Ok(())
}

fn show_local_commits(mut num: u8) -> Result<()> {
    if num == 0 {
        num = 5;
    }

    let adof_dir = get_adof_dir();
    env::set_current_dir(&adof_dir).context("Failed to change directory to adof dir")?;

    let default_branch = get_default_branch().context("Failed to get default branch")?;

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg("-n")
        .arg(num.to_string())
        .arg(default_branch)
        .output()
        .context("Failed to execute git log")?;

    let stdout = std::str::from_utf8(&output.stdout).context("Failed to read git log output")?;
    println!("{}", stdout);
    Ok(())
}

fn show_remote_commits(mut num: u8) -> Result<()> {
    if num == 0 {
        num = 5;
    }

    let adof_dir = get_adof_dir();
    env::set_current_dir(&adof_dir).context("Failed to change directory to adof dir")?;

    let remote_branch = "origin/main";

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg("-n")
        .arg(num.to_string())
        .arg(remote_branch)
        .output()
        .context("Failed to execute git log on remote")?;

    let stdout = std::str::from_utf8(&output.stdout).context("Failed to read git log output")?;
    println!("{}", stdout);
    Ok(())
}

fn show_only_local_commits() -> Result<()> {
    let adof_dir = get_adof_dir();
    env::set_current_dir(&adof_dir).context("Failed to change directory to adof dir")?;

    let default_branch = get_default_branch().context("Failed to get default branch")?;
    let diff_branch = format!("origin/main..{}", default_branch);

    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--color=always")
        .arg(diff_branch)
        .output()
        .context("Failed to execute git log for local-only commits")?;

    let stdout = std::str::from_utf8(&output.stdout).context("Failed to read git log output")?;
    println!("{}", stdout);
    Ok(())
}

fn get_only_local_commits_no() -> Result<u8> {
    let adof_dir = get_adof_dir();
    env::set_current_dir(&adof_dir).context("Failed to change directory to adof dir")?;

    let default_branch = get_default_branch().context("Failed to get default branch")?;
    let diff_branch = format!("origin/main..{}", default_branch);

    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(diff_branch)
        .output()
        .context("Failed to execute git rev-list")?;

    let count_str = std::str::from_utf8(&output.stdout)
        .context("Failed to read rev-list output")?
        .trim();

    let count = count_str
        .parse::<u8>()
        .context("Failed to parse commit count")?;
    Ok(count)
}

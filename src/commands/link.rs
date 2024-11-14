use crate::{commands::check_for_init, git::{is_remote_exist, remote::link_remote}};
use anyhow::{Context, Result};

pub fn link(repo_link: &str) -> Result<()> {
    if !check_for_init()? {
        eprintln!("Adof is not initialized.");
        std::process::exit(1);
    }
    
    if is_remote_exist().context("Failed to check if remote exists")? {
        println!("Remote branch is already configured.");
        std::process::exit(1);
    }

    link_remote(repo_link).context("Failed to link remote repository")?;
    println!("Remote linked, branch tracking configured, and commits pushed.");
    Ok(())
}

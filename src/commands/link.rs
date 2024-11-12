use crate::git::{is_remote_exist, remote::link_remote};
use anyhow::{Context, Result};

pub fn link(repo_link: &str) -> Result<()> {
    if is_remote_exist().context("Failed to check if remote exists")? {
        println!("Remote already exists");
        std::process::exit(1);
    }

    link_remote(repo_link).context("Failed to link remote repository")?;
    Ok(())
}

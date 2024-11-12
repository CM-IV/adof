use std::fs;

use anyhow::Context;

use super::*;

pub fn create_git_ignore() -> Result<()> {
    let adof_dir = adof::get_adof_dir();
    let git_ignore_file = format!("{}/.gitignore", adof_dir);
    fs::write(&git_ignore_file, b"./do_not_touch/pid.txt")
        .context("Failed to crate the .gitignore file")?;
    Ok(())
}

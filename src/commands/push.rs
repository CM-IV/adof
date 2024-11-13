use anyhow::{Context, Result};
use std::env;
use std::process::Command;

use adof::get_adof_dir;

pub fn push() -> Result<()> {
    let adof_dir = get_adof_dir();
    env::set_current_dir(&adof_dir).context("Failed to change directory to adof dir")?;

    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .context("Failed to execute git push")?;

    if output.status.success() {
        println!("Git push successful!");
    } else {
        eprintln!("Git push failed!");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("Git push command failed"));
    }

    Ok(())
}

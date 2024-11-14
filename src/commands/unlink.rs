use crate::git::{is_remote_exist, remote::unlink_remote};
use anyhow::{Context, Result};

use super::check_for_init;

pub fn unlink() -> Result<()> {
    if !check_for_init()? {
        eprintln!("Adof is not initialized.");
        std::process::exit(1);
    }

    if is_remote_exist().context("Checking if remote exists")? {
        unlink_remote()?;
        println!("You have successfully removed the remote branch.");
    } else {
        println!("Remote branch is not configured.");
    }
    Ok(())
}

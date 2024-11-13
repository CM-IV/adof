use crate::git::{is_remote_exist, remote::unlink_remote};
use anyhow::{Context, Result};

pub fn unlink() -> Result<()> {
    if is_remote_exist().context("Checking if remote exists")? {
        unlink_remote()?;
        println!("You have successfully removed the remote branch.");
    } else {
        println!("Remote branch is not configured.");
    }
    Ok(())
}

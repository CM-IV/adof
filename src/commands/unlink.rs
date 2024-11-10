use crate::git::{is_remote_exist, remote::unlink_remote};

use super::*;

pub fn unlink() -> Result<()> {
    if is_remote_exist() {
        unlink_remote()?;
    } else {
        println!("first connect to remote");
    }

    Ok(())
}

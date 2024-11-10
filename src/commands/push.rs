use std::env;
use std::process::Command;

use adof::get_adof_dir;

pub fn push() -> Result<()> {
    let adof_dir = get_adof_dir()?;
    env::set_current_dir(adof_dir)?;

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()?;
    Ok(())
}

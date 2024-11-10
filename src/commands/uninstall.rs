use std::fs;
use std::path::Path;
use std::process::Command;

use adof::{get_adof_dir, get_home_dir};

use super::*;

pub fn uninstall() -> Result<()> {
    let adof_dir = get_adof_dir()?;
    let pid_file = format!("{}/do_not_touch/pid.txt", adof_dir);

    if fs::exists(&pid_file)? {
        let pid = fs::read_to_string(&pid_file)?;
        Command::new("kill").arg(pid.trim()).output()?;
    }

    let home_dir = get_home_dir()?;
    let dotfile_readme_dir = format!("{}/dotfiles_readme", home_dir);
    remove_dir(&dotfile_readme_dir);

    let adof_dir = get_adof_dir()?;
    remove_dir(&adof_dir);

    let output = Command::new("which").arg("adof").output()?;
    let adof_bin_dir = String::from_utf8(output.stdout)?;
    remove_dir(adof_bin_dir.trim());

    Ok(())
}

fn remove_dir(dir: &str) -> Result<()> {
    let path = Path::new(dir);

    if path.is_dir() {
        fs::remove_dir_all(path)?;
    }

    if path.is_file() {
        fs::remove_file(path)?;
    }

    Ok(())
}

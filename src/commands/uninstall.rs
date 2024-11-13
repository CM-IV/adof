use adof::{get_adof_dir, get_home_dir};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn uninstall() -> Result<()> {
    let adof_dir = get_adof_dir();

    let home_dir = get_home_dir();
    let dotfile_readme_dir = format!("{}/dotfiles_readme", home_dir);
    remove_dir(&dotfile_readme_dir)?;

    remove_dir(&adof_dir)?;

    let output = Command::new("which")
        .arg("adof")
        .output()
        .context("Locating the adof binary")?;
    let adof_bin_dir =
        String::from_utf8(output.stdout).context("Converting binary path to UTF-8")?;
    remove_dir(adof_bin_dir.trim())?;

    Ok(())
}

fn remove_dir(dir: &str) -> Result<()> {
    let path = Path::new(dir);

    if path.is_dir() {
        fs::remove_dir_all(path).context(format!("Removing directory {:?}", dir))?;
    }

    if path.is_file() {
        fs::remove_file(path).context(format!("Removing file {:?}", dir))?;
    }
    Ok(())
}

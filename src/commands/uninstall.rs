use std::fs;
use std::path::Path;
use std::process::Command;

use adof::{get_adof_dir, get_home_dir};

pub fn uninstall() {
    let home_dir = get_home_dir();
    let dotfile_readme_dir = format!("{}/dotfiles_readme", home_dir);
    remove_dir(&dotfile_readme_dir);

    let adof_dir = get_adof_dir();
    remove_dir(&adof_dir);

    let output = Command::new("which").arg("adof").output().unwrap();
    println!("output: {:?}", output);
    let adof_bin_dir = String::from_utf8(output.stdout).unwrap();
    remove_dir(adof_bin_dir.trim());
}

fn remove_dir(dir: &str) {
    let path = Path::new(dir);
    fs::remove_dir_all(path).unwrap();
    println!("{:?}", dir);
}

use std::fs;
use std::path::Path;
use std::process::Command;

use adof::{get_adof_dir, get_home_dir};

pub fn uninstall() {
        let adof_dir = get_adof_dir();
    let pid_file = format!("{}/do_not_touch/pid.txt", adof_dir);
    let pid = fs::read_to_string(&pid_file).unwrap();
    Command::new("kill").arg(pid.trim()).output().unwrap();

    let home_dir = get_home_dir();
    let dotfile_readme_dir = format!("{}/dotfiles_readme", home_dir);
    remove_dir(&dotfile_readme_dir);

    let adof_dir = get_adof_dir();
    remove_dir(&adof_dir);

    let output = Command::new("which").arg("adof").output().unwrap();
    let adof_bin_dir = String::from_utf8(output.stdout).unwrap();
    remove_dir(adof_bin_dir.trim());
}

fn remove_dir(dir: &str) {
    let path = Path::new(dir);

    if path.is_dir() {
        fs::remove_dir_all(path).unwrap();
    }

    if path.is_file() {
        fs::remove_file(path).unwrap();
    }
}

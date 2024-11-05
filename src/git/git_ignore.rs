use std::fs;

use adof::get_adof_dir;

pub fn create_git_ignore() {
    let adof_dir = get_adof_dir();
    let git_ignore_file = format!("{}/.gitignore", adof_dir);
    fs::write(&git_ignore_file, b"./do_not_touch/pid.txt").unwrap();
}

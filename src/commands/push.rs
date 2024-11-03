use std::process::Command;
use std::env;

use adof::get_adof_dir;

pub fn push() {
    let adof_dir = get_adof_dir();
    env::set_current_dir(adof_dir).unwrap();

    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output();
    println!("{:?}", output);
}

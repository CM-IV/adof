use std::env;

pub fn get_home_dir() -> String {
    env::var("HOME").expect("Failed to get the home dir.")
}
pub fn get_adof_dir() -> String {
    let home_dir = get_home_dir();
    let adof_dir = format!("{}/{}", home_dir, ".adof");
    adof_dir
}

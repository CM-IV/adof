use std::env;

pub fn get_home_dir() -> String {
    let home_dir = env::var("HOME").expect("Failed to get the home dir.");
    home_dir
}
pub fn get_adof_dir() -> String {
    let home_dir = get_home_dir();
    let adof_dir = format!("{}/{}", home_dir, ".adof");
    adof_dir
}


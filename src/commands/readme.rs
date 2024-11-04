use std::fs;
use std::io;

use reqwest::blocking;

use adof::get_home_dir;

pub fn create_readme() -> String {
    let home_dir = get_home_dir();

    let readme_dir = format!("{}/dotfiles_readme", home_dir);
    fs::create_dir_all(&readme_dir).unwrap();

    let readme_file_path = format!("{}/README.md", readme_dir);
    let mut readme_file = fs::File::create(&readme_file_path).unwrap();

    let url = "https://raw.githubusercontent.com/fnabinash/rust-practice/refs/heads/main/README.md";
    let mut response = blocking::get(url).unwrap();

    io::copy(&mut response, &mut readme_file).unwrap();

    readme_file_path
}

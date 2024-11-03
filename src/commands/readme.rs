use std::fs;

use adof::get_home_dir;

pub fn create_readme() -> String {
    let home_dir = get_home_dir();

    let readme_dir = format!("{}/dotfiles_readme", home_dir);
    fs::create_dir_all(&readme_dir).unwrap();

    let readme_file_path = format!("{}/README.md", readme_dir);
    fs::File::create(&readme_file_path).unwrap();

    fs::copy("src/commands/README.md", &readme_file_path).unwrap();

    readme_file_path
}

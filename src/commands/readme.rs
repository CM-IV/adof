use std::fs;

use adof::{get_home_dir, get_adof_dir};

use crate::database::add;

pub async fn create_readme() {
    let local_readme = create_local_readme().await;
    create_backup_readme(&local_readme);
}

async fn create_local_readme() -> String {
    let home_dir = get_home_dir();
    let local_readme_dir = format!("{}/dotfiles_readme", home_dir);

    fs::create_dir_all(&local_readme_dir).unwrap();

    let local_readme_file_path = format!("{}/README.md", local_readme_dir);
    fs::File::create(&local_readme_file_path).unwrap();

    let url =
        "https://raw.githubusercontent.com/fnabinash/adof/refs/heads/main/src/commands/README.md";
    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    fs::write(&local_readme_file_path, response.as_bytes()).unwrap();

    local_readme_file_path
}

fn create_backup_readme(local_readme_file: &str) {
    let adof_dir = get_adof_dir();
    let backup_readme_file = format!("{}/README.md", adof_dir);

    fs::File::create(&backup_readme_file).unwrap();
    fs::copy(local_readme_file, &backup_readme_file).unwrap();

    add::add_files(local_readme_file, &backup_readme_file);
}

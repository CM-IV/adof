use std::fs;
use std::path::PathBuf;

use glob::glob;

use adof::get_home_dir;

use crate::commands::patterns::FILE_PATTERNS;
use crate::commands::readme::create_readme;
use crate::{
    database::{add, create_database},
    git::init_git,
};

use super::*;

pub fn init() {
    create_database();
    let readme_file_path = create_readme();

    let found_files = find_files();

    let mut selected_files = select_files(found_files);
    selected_files.push(readme_file_path);

    create_backup_files(&selected_files);
    init_git();
}

fn find_files() -> Vec<PathBuf> {
    let home_dir = get_home_dir();

    let mut found_files = Vec::new();

    for pattern in FILE_PATTERNS {
        let pattern_path = format!("{}/{}", home_dir, pattern);

        for entry in glob(&pattern_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => found_files.push(path),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    found_files
}

fn create_backup_files(selected_files: &[String]) {
    (0..selected_files.len()).for_each(|i| {
        let backup_file = create_file(&selected_files[i]);
        fs::copy(&selected_files[i], &backup_file).unwrap();
        add::add_files(&selected_files[i], &backup_file);
    })
}

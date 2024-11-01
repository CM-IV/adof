use std::fs;

use glob::glob;

use adof::get_home_dir;
use crate::database::add::add_files_to_database;
use crate::git::add::git_add;
use super::*;

pub fn add() {
    let files_to_add = get_files_to_add();
    create_backup_files(&files_to_add);
    git_add();
}

fn get_files_to_add() -> Vec<String> {
    let home_dir = get_home_dir();
    let pattern = format!("{}/**/*", home_dir);

    let mut found_files = Vec::new();

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    found_files.push(path);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    select_files(found_files)
}

fn create_backup_files(files_to_add: &Vec<String>) {
    (0..files_to_add.len()).for_each(|i| {
        let backup_file = create_backup_file(&files_to_add[i]);
        fs::copy(&files_to_add[i], &backup_file).unwrap();
        add_files_to_database(&files_to_add[i], &backup_file);
    })
}

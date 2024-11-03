use std::fs;
use std::process;

use glob::glob;

use adof::get_home_dir;

use crate::database::add::add_files;
use crate::git::add::git_add;

use super::*;

pub fn add() {
    let files_to_add = get_files_to_add();

    if files_to_add.is_empty() {
        println!("files are already exist");
        process::exit(1);
    }

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

    let selected_files = select_files(found_files);

    selected_files
        .into_iter()
        .filter(|file| !is_file_backedup(file))
        .collect::<Vec<String>>()
}

fn create_backup_files(files_to_add: &[String]) {
    (0..files_to_add.len()).for_each(|i| {
        let backup_file = create_file(&files_to_add[i]);
        fs::copy(&files_to_add[i], &backup_file).unwrap();
        add_files(&files_to_add[i], &backup_file);
    })
}

use glob::glob;

use crate::database::remove::remove_files;
use crate::git::add::git_add;

use super::*;

pub fn remove() {
    let files_to_remove = get_files_to_remove();
    remove_selected_files(&files_to_remove);
    git_add();
}

fn get_files_to_remove() -> Vec<String> {
    let adof_dir = get_adof_dir();
    let pattern = format!("{}/**/*", adof_dir);

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

fn remove_selected_files(files_to_remove: &[String]) {
    files_to_remove.iter().for_each(|file| {
        remove_files(file);
    })
}

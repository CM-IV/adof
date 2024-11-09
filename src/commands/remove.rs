use std::path::Path;

use crate::database::remove::remove_files;
use crate::git::add::git_add;

use super::*;

pub fn remove() {
    let files_to_remove = get_files_to_remove();
    remove_selected_files(&files_to_remove);
    git_add();
}

fn get_files_to_remove() -> Vec<String> {
    let mut found_files = Vec::new();

    let table_struct = get_table_struct();
    table_struct.table.values().for_each(|backedup_file| {
        found_files.push(backedup_file.into());
    });

    select_files(found_files)
}

fn remove_selected_files(files_to_remove: &[String]) {
    files_to_remove.iter().for_each(|file| {
        remove_files(file);
        remove_dir(file);
    })
}

fn remove_dir(file: &str) {
    if let Some(dir) = Path::new(file).parent() {
        if is_dir_empty(dir) {
            fs::remove_dir(dir).expect("Failed to remove empty directory");
            remove_dir(&dir.display().to_string());
        }
    }
}

fn is_dir_empty(dir: &Path) -> bool {
    match fs::read_dir(dir) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}

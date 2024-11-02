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
    })
}

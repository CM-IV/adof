use std::fs;

use crate::database::get_table_struct;
use crate::git::add::git_add;

pub fn update(check: bool) {
    let mut files_to_update: Vec<(String, String)> = Vec::new();

    let table_struct = get_table_struct();
    table_struct.table.iter().for_each(|data| {
        if is_to_modify(data.0, data.1) {
            files_to_update.push((data.0.to_string(), data.1.to_string()));
        }
    });

    if files_to_update.is_empty() {
        println!("Nothing changed");
    } else if check {
        show_files_to_update(&files_to_update);
    } else {
        show_files_to_update(&files_to_update);
        update_changes(&files_to_update);
    }
}

fn is_to_modify(original_file: &str, backedup_file: &str) -> bool {
    let original_file_metadata = fs::metadata(original_file).unwrap();
    let backedup_file_metadata = fs::metadata(backedup_file).unwrap();

    let original_file_last_modification = original_file_metadata.modified().unwrap();
    let backedup_file_last_modification = backedup_file_metadata.modified().unwrap();

    original_file_last_modification > backedup_file_last_modification
}

fn show_files_to_update(files_to_update: &[(String, String)]) {
    files_to_update.iter().for_each(|(original_file, _)| {
        println!("{:?}", original_file);
    });
}

fn update_changes(files_to_update: &[(String, String)]) {
    files_to_update
        .iter()
        .for_each(|(original_file, backedup_file)| {
            fs::copy(original_file, backedup_file).unwrap();
        });

    git_add();
}

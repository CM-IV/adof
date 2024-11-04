use std::fs;
use std::process;
use std::time::Duration;

use tokio::time::sleep;

use adof::get_adof_dir;

use crate::database::get_table_struct;
use crate::git::add::git_add;

pub async fn auto_update() {
    store_pid();

    loop {
        update();
        sleep(Duration::from_secs(10)).await;
    }
}

fn update() {
    let mut files_to_update: Vec<(String, String)> = Vec::new();

    let table_struct = get_table_struct();
    table_struct.table.iter().for_each(|data| {
        if is_to_modify(data.0, data.1) {
            files_to_update.push((data.0.to_string(), data.1.to_string()));
        }
    });

    files_to_update
        .iter()
        .for_each(|(original_file, backedup_file)| {
            fs::copy(original_file, backedup_file).unwrap();
        });

    if files_to_update.is_empty() {
        git_add();
    }
}

fn is_to_modify(original_file: &str, backedup_file: &str) -> bool {
    let original_file_metadata = fs::metadata(original_file).unwrap();
    let backedup_file_metadata = fs::metadata(backedup_file).unwrap();

    let original_file_last_modification = original_file_metadata.modified().unwrap();
    let backedup_file_last_modification = backedup_file_metadata.modified().unwrap();

    original_file_last_modification > backedup_file_last_modification
}

fn store_pid() {
    let adof_dir = get_adof_dir();
    let pid_file = format!("{}/do_not_touch/pid.txt", adof_dir);
    let pid = process::id();
    fs::write(&pid_file, pid.to_string()).unwrap();
}

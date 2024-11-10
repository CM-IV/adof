use std::fs;
use std::path::Path;
use std::process;
use std::time::Duration;

use tokio::time::sleep;

use super::*;

use crate::database::get_table_struct;
use crate::git::add::git_add;

#[allow(unreachable_code)]
pub async fn auto_update(min: u64) -> Result<()> {
    if Path::new(&get_pid_file()?).exists() {
        return Ok(())
    }

    store_pid()?;

    loop {
        update()?;
        sleep(Duration::from_secs(min * 60)).await;
    }

    delete_pid_file()?;
}

fn update() -> Result<()> {
    let mut files_to_update: Vec<(String, String)> = Vec::new();

    let table_struct = get_table_struct();
    table_struct.table.iter().for_each(|data| {
        if is_to_modify(data.0, data.1)? {
            files_to_update.push((data.0.to_string(), data.1.to_string()));
        }
    });

    if !files_to_update.is_empty() {
        files_to_update
            .iter()
            .for_each(|(original_file, backedup_file)| {
                fs::copy(original_file, backedup_file)?;
            });

        git_add();
    }
}

fn is_to_modify(original_file: &str, backedup_file: &str) -> Result<bool> {
    let original_hash = calculate_file_hash(original_file)?;
    let backup_hash = calculate_file_hash(backedup_file)?;
    Ok(original_hash != backup_hash)
}

fn store_pid() -> Result<()> {
    let pid_file = get_pid_file();
    let pid = process::id();
    fs::write(&pid_file, pid.to_string())?;
    Ok(())
}

fn delete_pid_file() -> Result<()> {
    fs::remove_file(get_pid_file())?;
    Ok(())
}

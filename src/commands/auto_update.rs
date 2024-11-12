use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;
use std::{fs, process};
use tokio::time::sleep;

use super::*;
use crate::database::get_table_struct;
use crate::git::add::git_add;

#[allow(unreachable_code)]
pub async fn auto_update(min: u64) -> Result<()> {
    if Path::new(&get_pid_file()).exists() {
        return Ok(()); // Early return if PID file exists
    }

    store_pid().context("Failed to store PID")?;

    loop {
        update().context("Failed to update files")?;
        sleep(Duration::from_secs(min * 60)).await;
    }

    delete_pid_file().context("Failed to delete PID file")?;
    Ok(())
}

fn update() -> Result<()> {
    let mut files_to_update = Vec::new();

    let table_struct = get_table_struct().context("Failed to retrieve table structure")?;
    for (original_file, backedup_file) in &table_struct.table {
        if is_to_modify(original_file, backedup_file).with_context(|| {
            format!(
                "Failed to compare file {} with backup {}",
                original_file, backedup_file
            )
        })? {
            files_to_update.push((original_file.clone(), backedup_file.clone()));
        }
    }

    if !files_to_update.is_empty() {
        for (original_file, backedup_file) in &files_to_update {
            fs::copy(original_file, backedup_file).with_context(|| {
                format!(
                    "Failed to copy file from {} to {}",
                    original_file, backedup_file
                )
            })?;
        }

        git_add().context("Failed to add files to git")?;
    }

    Ok(())
}

fn is_to_modify(original_file: &str, backedup_file: &str) -> Result<bool> {
    let original_hash = calculate_file_hash(original_file).with_context(|| {
        format!(
            "Failed to calculate hash for original file: {}",
            original_file
        )
    })?;
    let backup_hash = calculate_file_hash(backedup_file).with_context(|| {
        format!(
            "Failed to calculate hash for backup file: {}",
            backedup_file
        )
    })?;
    Ok(original_hash != backup_hash)
}

fn store_pid() -> Result<()> {
    let pid_file = get_pid_file();
    let pid = process::id();
    fs::write(&pid_file, pid.to_string())
        .with_context(|| format!("Failed to write PID to file: {}", pid_file))
}

fn delete_pid_file() -> Result<()> {
    fs::remove_file(get_pid_file()).context("Failed to remove PID file")
}

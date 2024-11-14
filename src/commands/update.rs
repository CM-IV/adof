use anyhow::{Context, Result};
use std::fs;

use super::*;
use crate::database::get_table_struct;
use crate::git::add::git_add;

pub fn update(check: bool) -> Result<()> {
    if !check_for_init()? {
        eprintln!("Adof is not initialized.");
        std::process::exit(1);
    }

    let mut files_to_update: Vec<(String, String)> = Vec::new();

    let table_struct = get_table_struct()?;
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

    if files_to_update.is_empty() {
        println!("Nothing changed");
    } else if check {
        show_files_to_update(&files_to_update);
    } else {
        show_files_to_update(&files_to_update);
        update_changes(&files_to_update)?;
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

fn show_files_to_update(files_to_update: &[(String, String)]) {
    files_to_update.iter().for_each(|(original_file, _)| {
        println!("{:?}", original_file);
    });
}

fn update_changes(files_to_update: &[(String, String)]) -> Result<()> {
    for (original_file, backedup_file) in files_to_update {
        fs::copy(original_file, backedup_file).context(format!(
            "Copying changes from {:?} to {:?}",
            original_file, backedup_file
        ))?;
    }

    git_add()?;
    Ok(())
}

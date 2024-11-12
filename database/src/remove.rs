use std::fs;

use anyhow::anyhow;

use super::*;
use crate::get::*;

pub fn remove_files(backup_file: &str) -> Result<()> {
    let home_dir = get_home_dir();
    let adof_dir = get_adof_dir();
    let original_file = backup_file.replace(&adof_dir, &home_dir);

    fs::remove_file(backup_file)
        .with_context(|| anyhow!("Failed to remove the file {:?}.", &original_file))?;

    let database_path = get_database_path();
    let mut table_struct = get_table_struct()?;

    table_struct
        .table
        .remove(&original_file)
        .with_context(|| anyhow!("Failed to remove the file {:?}.", &original_file))?;

    let json_table = serde_json::to_string_pretty(&table_struct)
        .context("Something went wrong. Please try again.")?;
    fs::write(&database_path, json_table)
        .context("Failed to update the database. Please try again!")?;
    Ok(())
}

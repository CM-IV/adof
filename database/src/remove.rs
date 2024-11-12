use std::fs;

use anyhow::Result;
use serde_json;

use crate::error::DBError;
use crate::get::*;

pub fn remove_files(backup_file: &str) -> Result<()> {
    let home_dir = get_home_dir();
    let adof_dir = get_adof_dir();

    fs::remove_file(backup_file).map_err(|e| DBError::FileError {
        file: backup_file.to_string(),
        source: e,
    })?;
    let original_file = backup_file.replace(&adof_dir, &home_dir);

    let database_path = get_database_path();
    let mut table_struct = get_table_struct()?;

    table_struct
        .table
        .remove(&original_file)
        .expect("Failed to remove the file. Please try again!");

    let json_table = serde_json::to_string_pretty(&table_struct)
        .expect("Something went wrong. Please try again.");
    fs::write(&database_path, json_table).expect("Failed to remove the file. Please try again!");
    Ok(())
}

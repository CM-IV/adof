use std::fs;

use anyhow::Result;

use super::DataTable;
use crate::error::DBError;
use crate::get::*;

pub fn add_files(original_path: &str, copied_path: &str) -> Result<()> {
    let database_path = get_database_path();

    let mut table_struct: DataTable = get_table_struct()?;

    table_struct
        .table
        .entry(original_path.to_string())
        .or_insert(copied_path.to_string());

    let json_table = serde_json::to_string_pretty(&table_struct)
        .expect("Something went wrong. Please try again.");
    fs::write(&database_path, json_table).map_err(|e| DBError::FileError {
        source: e,
        file: database_path.to_string(),
    })?;
    Ok(())
}

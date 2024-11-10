use std::fs;

use serde_json;

use adof::get_home_dir;

use super::*;

pub fn remove_files(backup_file: &str) -> Result<()> {
    let home_dir = get_home_dir()?;
    let adof_dir = get_adof_dir()?;

    fs::remove_file(backup_file)?;
    let original_file = backup_file.replace(&adof_dir, &home_dir);

    let database_path = get_database_path()?;
    let mut table_struct = get_table_struct()?;

    table_struct.table.remove(&original_file)?;

    let json_table = serde_json::to_string_pretty(&table_struct)?;
    fs::write(&database_path, json_table)?;

    Ok(())
}

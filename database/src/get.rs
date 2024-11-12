use std::fs;
use std::env;

use anyhow::Result;

use crate::error::DBError;
use crate::DataTable;

pub fn get_home_dir() -> String {
    env::var("HOME").expect("Failed to get the home dir.")
}

pub fn get_adof_dir() -> String {
    let home_dir = get_home_dir();
    format!("{}/{}", home_dir, ".adof")
}

pub fn get_database_path() -> String {
    let adof_dir = get_adof_dir();
    let database_path = format!("{}/{}", adof_dir, "do_not_touch/path_databse.json");

    database_path
}

pub fn get_table_struct() -> Result<DataTable> {
    let database_path = get_database_path();

    let database_contents = fs::read_to_string(&database_path).map_err(|e| DBError::FileError {
        file: database_path.to_string(),
        source: e,
    })?;

    let table_struct: DataTable =
        serde_json::from_str(&database_contents).map_err(|_| DBError::UnknownIssue)?;

    Ok(table_struct)
}

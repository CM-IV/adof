use std::collections::HashMap;
use std::fs;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub mod add;
pub mod create;
pub mod remove;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataTable {
    pub table: HashMap<String, String>,
}

impl DataTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

pub fn get_database_path() -> String {
    let adof_dir = adof::get_adof_dir();
    let database_path = format!("{}/{}", adof_dir, "do_not_touch/path_databse.json");

    database_path
}

pub fn get_table_struct() -> Result<DataTable> {
    let database_path = get_database_path();

    let database_contents = fs::read_to_string(&database_path).map_err(|e| {
        anyhow!(
            "Failed to read the file: {:?}. Source: {e:?}",
            database_path
        )
    })?;

    let table_struct: DataTable = serde_json::from_str(&database_contents)
        .context("Something went wrong. Please try again.")?;

    Ok(table_struct)
}

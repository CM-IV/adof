use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use adof::get_adof_dir;

pub mod add;
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

pub fn create_database() -> Result<()> {
    let database_path = get_database_path()?;
    let database_dir = Path::new(&database_path).parent()?;
    fs::create_dir_all(database_dir)?;

    fs::File::create(&database_path)?;

    let table_struct = DataTable::new();
    let json_table = serde_json::to_string_pretty(&table_struct)?;

    fs::write(&database_path, json_table)?;
    Ok(())
}

pub fn get_database_path() -> Result<String> {
    let adof_dir = get_adof_dir()?;
    let database_path = format!("{}/{}", adof_dir, "do_not_touch/path_databse.json");

    Ok(database_path)
}

pub fn get_table_struct() -> Result<DataTable> {
    let database_path = get_database_path()?;
    let database_contents = fs::read_to_string(&database_path)?;
    let table_struct: DataTable = serde_json::from_str(&database_contents)?;
    Ok(table_struct)
}

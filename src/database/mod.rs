use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

pub fn create_database() {
    let database_path = get_database_path();
    let database_dir = Path::new(&database_path).parent().unwrap();
    fs::create_dir_all(&database_dir).unwrap();

    fs::File::create(&database_path).unwrap();

    let table_struct = DataTable::new();
    let json_table = serde_json::to_string_pretty(&table_struct).unwrap();

    fs::write(&database_path, json_table).unwrap();
}

pub fn get_database_path() -> String {
    let adof_dir = get_adof_dir();
    let database_path = format!("{}/{}", adof_dir, "do_not_touch/path_databse.json");

    database_path
}

pub fn get_table_struct() -> DataTable {
    let database_path = get_database_path();
    let database_contents = fs::read_to_string(&database_path).unwrap();
    let table_struct: DataTable = serde_json::from_str(&database_contents).unwrap();
    table_struct
}

fn get_backup_file_path(original_path: &str) -> String {
    let table_struct = get_table_struct();
    let copied_path = table_struct.table.get(original_path).unwrap();
    copied_path.to_owned()
}

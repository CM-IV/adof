use std::fs;
use std::collections::HashMap;

use adof::get_adof_dir;

use serde::{Deserialize, Serialize};

pub mod add;
pub mod remove;

#[derive(Serialize, Deserialize, Debug)]
struct DataTable {
    table: HashMap<String, String>,
}

impl DataTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

fn get_database_path() -> String {
    let adof_dir = get_adof_dir();
    let database_dir = format!("{}/{}", adof_dir, "do_not_touch");

    fs::create_dir_all(&database_dir).expect("failed to create darabase dir");

    let database_path = format!("{}/{}", database_dir, "/path_databse.json");

    database_path
}

fn get_table_struct() -> DataTable {
    let database_path = get_database_path();
    let database_contents = fs::read_to_string(&database_path).unwrap();
    let table_struct: DataTable = serde_json::from_str(&database_contents).unwrap();
    table_struct
}

fn get_copied_file_path_by_key(original_path: &str) -> String {
    let table_struct = get_table_struct();
    let copied_path = table_struct.table.get(original_path).unwrap();
    copied_path.to_owned()
}

use std::fs;
use std::path::Path;

use super::DataTable;

use crate::get::*;

pub fn create_database() {
    let database_path = get_database_path();
    let database_dir = Path::new(&database_path).parent().expect("Failed to initiate the database");
    fs::create_dir_all(database_dir).expect("Failed to initiate the database");

    fs::File::create(&database_path).expect("Failed to initiate the database");

    let table_struct = DataTable::new();
    let json_table = serde_json::to_string_pretty(&table_struct).expect("Something went wrong. Please try again.");

    fs::write(&database_path, json_table).expect("Failed to initiate the database");
}

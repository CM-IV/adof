use std::fs;

use serde_json;

use super::*;

pub fn remove_files_from_database(original_path: &str) {
    let database_path = get_database_path();
    let mut table_struct = get_table_struct();
    
    let _ = table_struct.table.remove(original_path).unwrap();

    let json_table = serde_json::to_string_pretty(&table_struct).unwrap();
    fs::write(&database_path, json_table).unwrap();
}

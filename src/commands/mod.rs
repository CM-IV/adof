use std::collections::HashMap;
use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

pub mod init;

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

fn get_home_dir() -> String {
    let home_dir = env::var("HOME").expect("Failed to get the home dir.");
    home_dir
}

fn get_adof_dir() -> String {
    let home_dir = get_home_dir();
    let adof_dir = format!("{}/{}", home_dir, ".adof");
    adof_dir
}

fn create_adof_dir() {
    let adof_dir = get_adof_dir();
    fs::create_dir_all(&adof_dir).expect("failed to create adof dir.");
}

fn get_database_path() -> String {
    let adof_dir = get_adof_dir();
    let database_dir = format!("{}/{}", adof_dir, "do_not_touch");

    fs::create_dir_all(&database_dir).expect("failed to create darabase dir");

    let database_path = format!("{}/{}", database_dir, "/path_databse.json");

    database_path
}

fn create_database(database_path: &str) {
    fs::File::create(database_path).expect("Failed to create database.");
}

fn add_files_to_database(original_path: &str, copied_path: &str) {
    let database_path = get_database_path();

    if std::path::Path::new(&database_path).exists() {
        let mut table_struct: DataTable = get_table_struct();

        table_struct
            .table
            .entry(original_path.to_string())
            .or_insert(copied_path.to_string());

        let json_table = serde_json::to_string_pretty(&table_struct).unwrap();
        fs::write(&database_path, json_table).unwrap();
    } else {
        fs::File::create(&database_path).unwrap();

        let mut table_struct = DataTable::new();
        table_struct
            .table
            .entry(original_path.to_string())
            .or_insert(copied_path.to_string());
        let json_table = serde_json::to_string_pretty(&table_struct).unwrap();
        fs::write(&database_path, json_table).unwrap();
    }
}

fn remove_files_from_database(original_path: &str) {
    let database_path = get_database_path();
    let mut table_struct = get_table_struct();
    
    let _ = table_struct.table.remove(original_path).unwrap();

    let json_table = serde_json::to_string_pretty(&table_struct).unwrap();
    fs::write(&database_path, json_table).unwrap();
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

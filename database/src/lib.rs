use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod get;
pub mod create;

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    name: String,
    args: Vec<String>,
}

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

pub fn process_command(json_command: &str) -> Result<()> {
    let command: Command =
        serde_json::from_str(json_command).expect("Something went wrong. Please try again.");
    println!("{:?}", command.name);
    Ok(())
}

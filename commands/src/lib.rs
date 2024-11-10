use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    name: String,
    args: Vec<String>,
}

pub fn process_command(json_command: &str) {
    let command: Command = serde_json::from_str(json_command).expect("failed to deserialize");
    println!("{:?}", command.name);
}

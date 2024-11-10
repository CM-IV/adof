use std::env;

use anyhow::Result;

pub fn get_home_dir() -> Result<String> {
    let home_dir = env::var("HOME")?;
    Ok(home_dir)
}

pub fn get_adof_dir() -> String {
    let home_dir = get_home_dir();
    format!("{}/{}", home_dir, ".adof")
}

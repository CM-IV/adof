use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use adof::get_adof_dir;

pub fn list() -> Result<()> {
    let adof_dir = get_adof_dir();
    let path = Path::new(&adof_dir);
    println!("Root 📦 {}", path.display());
    print_directory(path, "")?;
    Ok(())
}

fn print_directory(path: &Path, prefix: &str) -> Result<()> {
    let entries = fs::read_dir(path)
        .context(format!("Failed to read directory: {}", path.display()))?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect directory entries")?;

    let len = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let is_last_entry = i == len - 1;

        if path.is_dir() {
            if path.file_name().unwrap() == ".git" {
                continue;
            }

            print_entry(&path, prefix, is_last_entry);
            let new_prefix = format!("{}{}", prefix, if is_last_entry { "    " } else { "│   " });
            print_directory(&path, &new_prefix)?;
        } else if path.is_file() {
            print_entry(&path, prefix, is_last_entry);
        }
    }
    Ok(())
}

fn print_entry(path: &Path, prefix: &str, is_last: bool) {
    let connector = if is_last { "└──" } else { "├──" };
    let name = path.file_name().unwrap().to_string_lossy();
    println!("{}{} {}", prefix, connector, name);
}

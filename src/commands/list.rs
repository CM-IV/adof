use std::fs;
use std::path::Path;

use adof::get_adof_dir;

use super::*;

pub fn list() -> Result<()> {
    let adof_dir = get_adof_dir()?;
    let path = Path::new(&adof_dir);
    println!("Root 📦 {}", path.display());
    print_directory(path, "")?;
    Ok(())
}

fn print_directory(path: &Path, prefix: &str) -> Result<()> {
    if let Ok(entries) = fs::read_dir(path) {
        let entries: Vec<_> = entries.collect::<Result<_, _>>()?;
        let len = entries.len();

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last_entry = i == len - 1;

            if path.is_dir() {
                if path.file_name()? == ".git" {
                    continue;
                }

                print_entry(&path, prefix, is_last_entry, true);
                let new_prefix =
                    format!("{}{}", prefix, if is_last_entry { "    " } else { "│   " });
                print_directory(&path, &new_prefix);
            } else if path.is_file() {
                print_entry(&path, prefix, is_last_entry, false);
            }
        }
    }
    Ok(())
}

fn print_entry(path: &Path, prefix: &str, is_last: bool, is_dir: bool) -> Result<()> {
    let connector = if is_last { "└──" } else { "├──" };
    let name = path.file_name()?.to_string_lossy();

    println!("{}{} {}", prefix, connector, name);

    Ok(())
}

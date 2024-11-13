use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn list() -> Result<()> {
    let path = Path::new("/home/abinash/.adof");
    println!("Root 📦 {}", path.display());
    print_directory(path, "");
    Ok(())
}

fn print_directory(path: &Path, prefix: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        let entries: Vec<_> = entries.filter_map(Result::ok)
                                     .filter(|entry| entry.file_name() != ".git")
                                     .collect();
        let len = entries.len();

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last_entry = i == len - 1;

            print_entry(&path, prefix, is_last_entry, path.is_dir());

            if path.is_dir() {
                let new_prefix = if is_last_entry {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                print_directory(&path, &new_prefix);
            }
        }
    }
}

fn print_entry(path: &Path, prefix: &str, is_last: bool, is_dir: bool) {
    let icon = if is_dir { "📂" } else { "📄" };
    let connector = if is_last { "└──" } else { "├──" };
    let name = path.file_name().unwrap().to_string_lossy();

    println!("{}{} {} {}", prefix, connector, icon, name);
}


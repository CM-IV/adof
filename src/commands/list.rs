use std::fs;
use std::path::Path;

pub fn list() {
    let path = Path::new("/home/abinash/.adof");
    println!("Root 📦 {}", path.display());
    print_directory(path, 0, "");
}

fn print_directory(path: &Path, level: usize, prefix: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        let entries: Vec<_> = entries.collect::<Result<_, _>>().unwrap();
        let len = entries.len();

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last_entry = i == len - 1;

            if path.is_dir() {
                if path.file_name().unwrap() == ".git" {
                    continue;
                }
                
                print_entry(&path, prefix, is_last_entry, true);
                let new_prefix = format!("{}{}", prefix, if is_last_entry { "    " } else { "│   " });
                print_directory(&path, level + 1, &new_prefix);
            } else if path.is_file() {
                print_entry(&path, prefix, is_last_entry, false);
            }
        }
    }
}

fn print_entry(path: &Path, prefix: &str, is_last: bool, is_dir: bool) {
    let icon = if is_dir { "📁" } else { "📄" };
    let connector = if is_last { "└──" } else { "├──" };
    let name = path.file_name().unwrap().to_string_lossy();

    println!("{}{} {} {}", prefix, connector, icon, name);
}

use std::fs;
use std::path::PathBuf;

use adof::get_adof_dir;

pub fn list() {
    let path = get_adof_dir();
    print_dir_tree(&path, 0, &mut Vec::new());
}

fn print_dir_tree(path: &str, depth: usize, stack: &mut Vec<bool>) {
    let entries = fs::read_dir(path).expect("Failed to read directory");
    let entries: Vec<_> = entries.map(|e| e.expect("Failed to read entry")).collect();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let entry_path = entry.path();

        if entry_path.is_dir() && entry_path.file_name().unwrap() == ".git" {
            continue;
        }

        print_entry(stack, entry_path.clone(), depth, is_last);

        if entry_path.is_dir() {
            print_dir_tree(entry_path.to_str().unwrap(), depth + 1, stack);
        }
    }

    if let Some(last) = stack.last_mut() {
        *last = true;
    }
}

fn print_entry(stack: &mut Vec<bool>, path: PathBuf, depth: usize, is_last: bool) {
    for i in 0..depth {
        if i < stack.len() {
            if stack[i] {
                print!("   ");
            } else {
                print!("│  ");
            }
        } else {
            print!("   ");
        }
    }

    if is_last {
        print!("└── ");
        if stack.len() > depth {
            stack.truncate(depth);
        }
        stack.push(true);
    } else {
        print!("├── ");
        if stack.len() > depth {
            stack.truncate(depth);
        }
        stack.push(false);
    }

    if path.is_dir() {
        println!("📁 {}", path.file_name().unwrap().to_string_lossy());
    } else {
        println!("📄 {}", path.file_name().unwrap().to_string_lossy());
    }
}


use std::collections::HashMap;

use chrono::Local;
use git2::DiffOptions;

use super::*;

fn get_current_date_and_time() -> String {
    let current_time = Local::now().naive_local();
    let formatted_current_time = current_time.format("%a,%e %b %Y%l:%M %p");
    formatted_current_time.to_string()
}

fn get_file_change_logs() {
    let repo = get_repo();

    let tree = match repo.head() {
        Ok(head_ref) => {
            let head_commit = head_ref.peel_to_commit().unwrap();
            Some(head_commit.tree().unwrap())
        }
        Err(_) => None,
    };

    let index = repo.index().unwrap();
    let mut diff_options = DiffOptions::new();
    let diff = repo
        .diff_tree_to_index(tree.as_ref(), Some(&index), Some(&mut diff_options))
        .unwrap();

    let mut file_changes: HashMap<String, (usize, usize)> = HashMap::new();

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(file_path) = delta.new_file().path() {
            let file_name = file_path.to_string_lossy().to_string();
            let entry = file_changes.entry(file_name).or_insert((0, 0));
            
            match line.origin() {
                '+' => entry.0 += 1,
                '-' => entry.1 += 1,
                _ => {}
            }
        }
        true
    }).unwrap();
}

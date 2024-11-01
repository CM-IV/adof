use std::collections::HashMap;

use chrono::Local;
use git2::{Signature, DiffOptions};

use super::*;

pub fn commit() {
    let current_time = get_current_date_and_time();
    let change_log = get_change_logs();
    let commit_message = get_commit_message(&current_time, &change_log);
}

fn get_current_date_and_time() -> String {
    let current_time = Local::now().naive_local();
    let formatted_current_time = current_time.format("%a,%e %b %Y%l:%M %p");
    formatted_current_time.to_string()
}

fn get_change_logs() -> String {
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

    let mut file_changes: HashMap<String, (usize, usize, usize)> = HashMap::new();

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(file_path) = delta.new_file().path() {
            let file_name = file_path.to_string_lossy().to_string();
            let entry = file_changes.entry(file_name).or_insert((0, 0, 0));
            
            match line.origin() {
                '+' => entry.0 += 1,
                '-' => entry.1 += 1,
                ' ' => entry.2 += 1,
                _ => {}
            }
        }
        true
    }).unwrap();

    let mut change_log = Vec::new();

    for (file, (added, removed, modified)) in file_changes {
        let message = format!("{} +{} -{} ~{}", file, added, removed, modified);
        change_log.push(message);
    }

    change_log.join("\n")
}

fn get_commit_message(current_time: &str, change_log: &str) -> String {
    format!("{}\n\n{}", current_time, change_log)
}

fn commit_changes(commit_message: &str) {
    let repo = get_repo();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let head_ref = repo.head().unwrap();
    let parent_commit = head_ref.peel_to_commit().unwrap();

    let signature = Signature::now("Your Name", "your.email@example.com").unwrap();
    let commit_id = repo.commit(
        Some("HEAD"), 
        &signature,
        &signature,
        commit_message,
        &tree,
        &[&parent_commit],
    ).unwrap();

    repo.set_head(&commit_id.to_string()).unwrap();

    println!("Committed changes with message: {}", commit_message);
}

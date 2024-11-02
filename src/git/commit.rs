use std::collections::HashMap;

use chrono::Local;
use git2::{Delta, DiffOptions, Signature};

use super::*;

pub fn commit() {
    let current_time = get_current_date_and_time();
    let each_file_status = get_change_logs();
    let commit_message = get_commit_message(&current_time, &each_file_status);
    commit_changes(&commit_message);
}

fn get_current_date_and_time() -> String {
    let current_time = Local::now().naive_local();
    let formatted_current_time = current_time.format("%a,%e %b %Y %l:%M %p");
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

    let mut added_files = HashMap::new();
    let mut removed_files = HashMap::new();
    let mut modified_files = HashMap::new();

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(file_path) = delta.new_file().path() {
            let file_name = file_path.to_string_lossy().to_string();
            let entry = match delta.status() {
                Delta::Added => added_files.entry(file_name).or_insert((0, 0, 0)),
                Delta::Deleted => removed_files.entry(file_name).or_insert((0, 0, 0)),
                Delta::Modified => modified_files.entry(file_name).or_insert((0, 0, 0)),
                _ => return true,
            };

            match line.origin() {
                '+' => entry.0 += 1,
                '-' => entry.1 += 1,
                ' ' => entry.2 += 1,
                _ => {}
            }

            true
        } else {
            false
        }
    })
    .unwrap();

    let change_log = vec![
        ("Files Added", added_files),
        ("Files Removed", removed_files),
        ("Files Modified", modified_files),
    ];

    let mut file_logs = Vec::new();
    for (change_type, files) in change_log {
        if !files.is_empty() {
            let mut change_message = format!("{}: {} file(s)\n", change_type, files.len());
            for (file_name, (added, removed, modified)) in files {
                change_message.push_str(&format!(
                    "  ▶ {} +{} -{} ~{}\n",
                    file_name, added, removed, modified
                ));
            }
            file_logs.push(change_message);
        }
    }

    file_logs.join("\n")
}

fn get_commit_message(current_time: &str, each_file_status: &str) -> String {
    format!("{}\n\n{}", current_time, each_file_status)
}

fn commit_changes(commit_message: &str) {
    let repo = get_repo();
    let mut index = repo.index().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let parent_commit = match repo.head() {
        Ok(head_ref) => Some(head_ref.peel_to_commit().unwrap()),
        Err(_) => None,
    };

    let config = repo.config().unwrap();
    let name = config
        .get_string("user.name")
        .unwrap_or("Unknown".to_string());
    let email = config
        .get_string("user.email")
        .unwrap_or("unknown@example.com".to_string());

    let signature = Signature::now(&name, &email).unwrap();

    if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent],
        )
        .unwrap()
    } else {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[],
        )
        .unwrap()
    };
}

use std::collections::HashMap;

use git2::{Delta, DiffOptions};

use super::*;

pub fn get_commit_message() -> Result<String> {
    let current_time = get_current_date_and_time();
    let each_file_status = get_change_logs()?;
    let commit_message = format!("{}\n\n{}", current_time, each_file_status);
    Ok(commit_message)
}

fn get_current_date_and_time() -> String {
    let current_time = Local::now().naive_local();
    let formatted_current_time = current_time.format("%a,%e %b %Y %l:%M %p");
    formatted_current_time.to_string()
}

fn get_change_logs() -> Result<String> {
    let repo = get_repo()?;

    let tree = match repo.head() {
        Ok(head_ref) => {
            let head_commit = head_ref.peel_to_commit()?;
            Some(head_commit.tree()?)
        }
        Err(_) => None,
    };

    let index = repo.index()?;
    let mut diff_options = DiffOptions::new();
    let diff = repo.diff_tree_to_index(tree.as_ref(), Some(&index), Some(&mut diff_options))?;

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
    })?;

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

    let file_logs = file_logs.join("\n");
    Ok(file_logs)
}

use std::collections::HashSet;

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

    let mut added_files = HashSet::new();
    let mut removed_files = HashSet::new();
    let mut modified_files = HashSet::new();

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(file_path) = delta.new_file().path() {
            let file_name = file_path.to_string_lossy().to_string();

            match delta.status() {
                Delta::Added => {
                    let mut file_changes = ("Files Added", file_name, 0, 0, 0);

                    match line.origin() {
                        '+' => file_changes.2 += 1,
                        _ => {}
                    }

                    added_files.insert(file_changes);
                }

                Delta::Deleted => {
                    let mut file_changes = ("Filed Removed", file_name, 0, 0, 0);

                    match line.origin() {
                        '-' => file_changes.3 += 1,
                        _ => {}
                    }

                    removed_files.insert(file_changes);
                }

                Delta::Modified => {
                    let mut file_changes = ("Files Modified", file_name, 0, 0, 0);

                    match line.origin() {
                        '+' => file_changes.2 += 1,
                        '-' => file_changes.3 += 1,
                        ' ' => file_changes.4 += 1,
                        _ => {}
                    }

                    modified_files.insert(file_changes);
                }
                _ => {}
            }
        }

        true
    })
    .unwrap();

    let change_log = vec![added_files, removed_files, modified_files];

    let mut file_logs = Vec::new();

    change_log.iter().for_each(|file_changes| {
        if file_changes.len() != 0 {
            file_changes.iter().for_each(|file_change| {
                let message = format!(
                    "{}: {}\n\t- {} +{} -{} ~{}",
                    file_change.0,
                    file_changes.len(),
                    file_change.1,
                    file_change.2,
                    file_change.3,
                    file_change.4
                );
                file_logs.push(message);
            });
        }
    });

    file_logs.join("\n\n")
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

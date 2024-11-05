use std::collections::HashMap;

use git2::{BranchType, Delta, DiffOptions, Oid, Repository};

use super::*;

pub fn merge() {
    let repo = get_repo();
    let old_branch = get_old_branch();
    let default_branch = get_default_branch();

    squash_merge(&repo, &old_branch, &default_branch);
    delete_old_branch(&old_branch);
}

fn get_commit_message(repo: &Repository, source_oid: Oid, target_oid: Oid) -> String {
    let target_commit = repo.find_commit(target_oid).unwrap();
    let source_commit = repo.find_commit(source_oid).unwrap();

    let target_tree = target_commit.tree().unwrap();
    let source_tree = source_commit.tree().unwrap();

    let mut diff_options = DiffOptions::new();
    let diff = repo
        .diff_tree_to_tree(
            Some(&target_tree),
            Some(&source_tree),
            Some(&mut diff_options),
        )
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

    let change_logs = file_logs.join("\n");
    let current_time = get_current_date_and_time();
    format!("{}\n\n{}", current_time, change_logs)
}

fn squash_merge(repo: &Repository, source_branch: &str, target_branch: &str) {
    repo.set_head(&format!("refs/heads/{}", target_branch))
        .unwrap();
    repo.checkout_head(None).unwrap();

    let target_oid = repo
        .refname_to_id(&format!("refs/heads/{}", target_branch))
        .unwrap();
    let source_oid = repo
        .refname_to_id(&format!("refs/heads/{}", source_branch))
        .unwrap();

    let commit_message = get_commit_message(repo, source_oid, target_oid);

    let target_commit = repo.find_commit(target_oid).unwrap();
    let source_commit = repo.find_commit(source_oid).unwrap();

    let target_tree = target_commit.tree().unwrap();
    let source_tree = source_commit.tree().unwrap();

    let diff = repo
        .diff_tree_to_tree(Some(&target_tree), Some(&source_tree), None)
        .unwrap();
    repo.apply(&diff, git2::ApplyLocation::WorkDir, None)
        .unwrap();

    let mut index = repo.index().unwrap();
    index
        .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();

    let signature = get_signature();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &[&target_commit],
    )
    .unwrap();
}

fn delete_old_branch(old_branch: &str) {
    get_repo()
        .find_branch(old_branch, BranchType::Local)
        .unwrap()
        .delete()
        .unwrap();
}

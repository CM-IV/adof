use std::collections::HashMap;

use git2::{Repository, Signature, DiffDelta, DiffOptions, Oid, BranchType};

use super::*;

pub fn merge() {
    let repo = get_repo();
    let old_branch = get_old_branch();
    let default_branch = get_default_branch();

    squash_merge(&repo, &old_branch, &default_branch);
    delete_old_branch(&old_branch);
}

fn get_old_branch() -> String {
    let repo = get_repo();
    let default_branch = get_default_branch();
    let mut old_branch = String::new();

    let branches_iter = repo.branches(Some(BranchType::Local)).unwrap();

    for branch in branches_iter {
        let (branch, _) = branch.unwrap();
        let branch_name = branch.name().unwrap().unwrap_or_default().to_string();

        if branch_name != default_branch {
            old_branch = branch_name;
        }
    }

    old_branch
}

fn delete_old_branch(old_branch: &str) {
    get_repo()
        .find_branch(old_branch, BranchType::Local)
        .unwrap()
        .delete()
        .unwrap();
}

fn get_detailed_commit_message(repo: &Repository, source_oid: Oid, target_oid: Oid) -> String {
    let source_commit = repo.find_commit(source_oid).unwrap();
    let target_commit = repo.find_commit(target_oid).unwrap();
    let source_tree = source_commit.tree().unwrap();
    let target_tree = target_commit.tree().unwrap();

    // Create a diff between the target and source branch
    let mut diff_options = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(Some(&target_tree), Some(&source_tree), Some(&mut diff_options)).unwrap();

    // Track added, modified, and removed files with line changes
    let mut added_files = HashMap::new();
    let mut removed_files = HashMap::new();
    let mut modified_files = HashMap::new();

    // Collect file changes
    diff.foreach(
        &mut |delta, _| {
            let path = delta.new_file().path().unwrap().display().to_string();
            match delta.status() {
                git2::Delta::Added => {
                    added_files.insert(path, (0, 0, 0)); // placeholder for lines added/removed/modified
                }
                git2::Delta::Deleted => {
                    removed_files.insert(path, (0, 0, 0));
                }
                git2::Delta::Modified => {
                    modified_files.insert(path, (0, 0, 0));
                }
                _ => (),
            }
            true
        },
        None,
        Some(&mut |delta, _diff_hunk, diff_line| {
            let path = delta.new_file().path().unwrap().display().to_string();
            let (added, removed, modified) = match delta.status() {
                git2::Delta::Added => added_files.get_mut(&path).unwrap(),
                git2::Delta::Deleted => removed_files.get_mut(&path).unwrap(),
                git2::Delta::Modified => modified_files.get_mut(&path).unwrap(),
                _ => return true,
            };

            match diff_line.origin() {
                '+' => *added += 1,
                '-' => *removed += 1,
                ' ' => *modified += 1,
                _ => (),
            }
            true
        }),
        None,
    ).unwrap();

    // Format the commit message
    let mut message = String::new();

    if !added_files.is_empty() {
        message.push_str(&format!("File Added: {} file(s)\n", added_files.len()));
        for (file, (add, remove, modif)) in &added_files {
            message.push_str(&format!("  > {} +{} -{} ~{}\n", file, add, remove, modif));
        }
    }

    if !removed_files.is_empty() {
        message.push_str(&format!("File Removed: {} file(s)\n", removed_files.len()));
        for (file, (add, remove, modif)) in &removed_files {
            message.push_str(&format!("  > {} +{} -{} ~{}\n", file, add, remove, modif));
        }
    }

    if !modified_files.is_empty() {
        message.push_str(&format!("File Modified: {} file(s)\n", modified_files.len()));
        for (file, (add, remove, modif)) in &modified_files {
            message.push_str(&format!("  > {} +{} -{} ~{}\n", file, add, remove, modif));
        }
    }

    message
}

fn squash_merge(repo: &Repository, source_branch: &str, target_branch: &str) {
    // Checkout the target branch
    repo.set_head(&format!("refs/heads/{}", target_branch)).unwrap();
    repo.checkout_head(None).unwrap();

    // Get the OIDs of the target and source branches
    let target_oid = repo.refname_to_id(&format!("refs/heads/{}", target_branch)).unwrap();
    let source_oid = repo.refname_to_id(&format!("refs/heads/{}", source_branch)).unwrap();

    // Get the detailed commit message for the squash merge
    let commit_message = get_detailed_commit_message(repo, source_oid, target_oid);

    // Apply the diff to the working directory
    let target_commit = repo.find_commit(target_oid).unwrap();
    let source_commit = repo.find_commit(source_oid).unwrap();
    let target_tree = target_commit.tree().unwrap();
    let source_tree = source_commit.tree().unwrap();
    let diff = repo.diff_tree_to_tree(Some(&target_tree), Some(&source_tree), None).unwrap();
    repo.apply(&diff, git2::ApplyLocation::WorkDir, None).unwrap();

    // Add the changes to the index
    let mut index = repo.index().unwrap();
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();

    // Commit the squash merge with the detailed message
    let sig = Signature::now("Your Name", "your.email@example.com").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, &commit_message, &tree, &[&target_commit]).unwrap();

}

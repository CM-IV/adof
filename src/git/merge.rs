use std::collections::HashMap;

use git2::{
    BranchType, CherrypickOptions, Delta, DiffOptions, IndexAddOption, MergeOptions, Oid,
    Repository,
};

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

fn squash_merge(repo: &Repository, source_branch_name: &str, target_branch_name: &str) {
    repo.set_head(&format!("refs/heads/{}", target_branch_name))
        .unwrap();
    repo.checkout_head(None).unwrap();

    let sig = get_signature();
    let target_branch = repo
        .find_branch(target_branch_name, BranchType::Local)
        .unwrap();
    let source_branch = repo
        .find_branch(source_branch_name, BranchType::Local)
        .unwrap();

    let target_oid = target_branch.get().target().unwrap();
    let source_oid = source_branch.get().target().unwrap();

    let target_commit = repo.find_commit(target_oid).unwrap();
    let source_commit = repo.find_commit(source_oid).unwrap();

    let _temp_branch = repo
        .branch("temp_squash", &repo.find_commit(target_oid).unwrap(), false)
        .unwrap();
    repo.set_head("refs/heads/temp_squash").unwrap();
    repo.checkout_head(None).unwrap();

    let mut merge_opts = MergeOptions::new();
    merge_opts.file_favor(git2::FileFavor::Theirs);

    repo.merge_commits(&target_commit, &source_commit, Some(&merge_opts))
        .unwrap();

    let mut index = repo.index().unwrap();

    if index.has_conflicts() {
        index
            .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
    }

    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();
    let commit_message = get_commit_message(&repo, source_oid, target_oid);

    let squash_commit_oid = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            &commit_message,
            &tree,
            &[&target_commit, &source_commit],
        )
        .unwrap();

    repo.set_head(&format!("refs/heads/{}", target_branch_name))
        .unwrap();
    repo.checkout_head(None).unwrap();
    let squash_commit = repo.find_commit(squash_commit_oid).unwrap();

    let mut cherry_pick_opts = CherrypickOptions::new();

    let mut merge_opts = MergeOptions::new();
    merge_opts.file_favor(git2::FileFavor::Theirs);

    cherry_pick_opts.mainline(1).merge_opts(merge_opts);

    if let Err(e) = repo.cherrypick(&squash_commit, Some(&mut cherry_pick_opts)) {
        if e.code() == git2::ErrorCode::MergeConflict {
            let mut index = repo.index().unwrap();
            index
                .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
                .unwrap();
            index.write().unwrap();

            let tree_oid = index.write_tree().unwrap();
            let tree = repo.find_tree(tree_oid).unwrap();
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &commit_message,
                &tree,
                &[&repo.head().unwrap().peel_to_commit().unwrap()],
            )
            .unwrap();
        }
    }

    repo.find_branch("temp_squash", BranchType::Local)
        .unwrap()
        .delete()
        .unwrap();
}

fn delete_old_branch(old_branch: &str) {
    get_repo()
        .find_branch(old_branch, BranchType::Local)
        .unwrap()
        .delete()
        .unwrap();
}

use crate::git::commit_message::get_commit_message;

use super::*;

pub fn commit() {
    let commit_message = get_commit_message();
    commit_changes(&commit_message);
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

    let signature = get_signature();

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

use crate::git::merge::merge;

use super::*;

pub fn create_new_branch() {
    merge();

    let repo = get_repo();
    let branch_name = get_today();

    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
    repo.branch(&branch_name, &head_commit, false).unwrap();

    let branch_ref = format!("refs/heads/{}", branch_name);
    repo.set_head(&branch_ref).unwrap();
    repo.checkout_head(None).unwrap();
}

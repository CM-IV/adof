use git2::IndexAddOption;

use crate::git::commit::commit;

use super::*;

pub fn git_add() {
    let repo = get_repo();
    let mut index = repo.index().unwrap();

    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .unwrap();

    index.write().unwrap();

    commit();
}

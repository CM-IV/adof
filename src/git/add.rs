use git2::IndexAddOption;

use super::*;

pub fn add() {
    let repo = get_repo();
    let mut index = repo.index().unwrap();
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
}

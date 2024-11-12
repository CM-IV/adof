use git2::IndexAddOption;

use crate::git::commit::commit;

use super::*;

pub fn git_add() -> Result<()> {
    let repo = get_repo()?;
    let mut index = repo.index().unwrap();

    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .context("Failed to stage the changes. Please try again!")?;

    index
        .write()
        .context("Failed to stage the changes. Please try again")?;

    commit()?;
    Ok(())
}

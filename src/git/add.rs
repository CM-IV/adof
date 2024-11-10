use git2::IndexAddOption;

use crate::git::commit::commit;

use super::*;

pub fn git_add() -> Result<()> {
    let repo = get_repo();
    let mut index = repo.index()?;

    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

    index.write()?;

    commit();
    Ok(())
}

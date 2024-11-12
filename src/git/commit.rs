use git2::Signature;

use crate::git::commit_message::get_commit_message;

use super::*;

pub fn commit() -> Result<()> {
    let commit_message = get_commit_message()?;
    commit_changes(&commit_message)?;
    Ok(())
}

fn get_signature() -> Result<Signature<'static>> {
    let repo = get_repo()?;
    let config = repo.config().context("Failed to load the Git config.")?;

    let name = config
        .get_string("user.name")
        .unwrap_or("Unknown".to_string());
    let email = config
        .get_string("user.email")
        .unwrap_or("unknown@example.com".to_string());

    Ok(Signature::now(&name, &email).unwrap())
}

fn commit_changes(commit_message: &str) -> Result<()> {
    let repo = get_repo()?;
    let mut index = repo.index().context("Failed to commit the changes.")?;

    let tree_id = index
        .write_tree()
        .context("Can not commit the latest changes")?;
    let tree = repo
        .find_tree(tree_id)
        .context("Failed to commit the changes")?;

    let parent_commit = match repo.head() {
        Ok(head_ref) => Some(
            head_ref
                .peel_to_commit()
                .context("Failed to commit the changes")?,
        ),
        Err(_) => None,
    };

    let signature = get_signature()?;

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

    Ok(())
}

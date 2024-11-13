use anyhow::{Context, Result};
use git2::{build::CheckoutBuilder, Oid, Repository};
use std::fs;
use std::path::Path;

use super::update::update;
use super::*;
use crate::git::{get_repo, init_git};

pub fn deploy(repo_link: &str, commit_id: &str) -> Result<()> {
    if repo_link.is_empty() {
        deploy_from_local(commit_id)?;
    } else {
        deploy_from_remote(repo_link, commit_id)?;
    }

    println!("Files deployed successfully.");
    Ok(())
}

fn deploy_from_local(commit_id: &str) -> Result<()> {
    if !commit_id.is_empty() {
        let repo = get_repo().context("Failed to retrieve the local repository")?;

        let original_head = repo.head().context("Failed to get the repository head")?;
        let original_commit = original_head
            .peel_to_commit()
            .context("Failed to peel to original commit")?;

        deploy_with_commit_id(commit_id)?;

        create_and_copy_files()?;

        let mut checkout_builder = CheckoutBuilder::new();
        repo.checkout_tree(
            original_commit
                .tree()
                .context("Failed to get tree from original commit")?
                .as_object(),
            Some(&mut checkout_builder),
        )
        .context("Failed to checkout original tree")?;

        repo.set_head("refs/heads/main")
            .context("Failed to set head to 'main'")?;

        update(false)?;
    } else {
        create_and_copy_files()?;
    }
    Ok(())
}

fn deploy_with_commit_id(commit_id: &str) -> Result<()> {
    let repo = get_repo().context("Failed to retrieve the local repository")?;

    let oid = Oid::from_str(commit_id).context("Failed to parse commit ID")?;
    let commit = repo
        .find_commit(oid)
        .context("Failed to find commit by ID")?;

    let tree = commit.tree().context("Failed to get tree from commit")?;
    let tree_obj = tree.as_object();

    let mut checkout_builder = CheckoutBuilder::new();
    repo.checkout_tree(tree_obj, Some(&mut checkout_builder))
        .context("Failed to checkout commit tree")?;

    repo.set_head_detached(oid)
        .context("Failed to set detached head for commit")?;
    Ok(())
}

fn deploy_from_remote(repo_link: &str, commit_id: &str) -> Result<()> {
    if check_for_init()? {
        empty_adof_dir().context("Failed to empty adof directory")?;
    }

    let adof_dir = adof::get_adof_dir();
    Repository::clone(repo_link, &adof_dir)
        .context("Failed to clone repository from remote link")?;

    if !commit_id.is_empty() {
        deploy_with_commit_id(commit_id)?;
    }

    create_and_copy_files()?;

    let git_dir = format!("{}/.git", adof_dir);
    fs::remove_dir_all(git_dir).context("Failed to remove .git directory")?;

    init_git().context("Failed to reinitialize git")?;
    Ok(())
}

fn empty_adof_dir() -> Result<()> {
    let adof_dir = adof::get_adof_dir();
    fs::remove_dir_all(&adof_dir).context("Failed to remove adof directory")?;
    Ok(())
}

fn create_and_copy_files() -> Result<()> {
    let table_struct = get_table_struct()?;

    for (original_file, backup_file) in &table_struct.table {
        let original_path = Path::new(original_file);

        if !original_path.exists() {
            let path_dir = original_path
                .parent()
                .context("Failed to get parent directory")?;
            fs::create_dir_all(path_dir).context("Failed to create directories for file")?;
            fs::File::create(original_file).context("Failed to create original file")?;
        }

        fs::copy(backup_file, original_file).with_context(|| {
            format!(
                "Failed to copy backup file {} to original file {}",
                backup_file, original_file
            )
        })?;
    }
    Ok(())
}

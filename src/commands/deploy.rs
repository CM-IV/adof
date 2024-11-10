use std::fs;
use std::path::Path;

use git2::{build::CheckoutBuilder, Oid, Repository};

use crate::git::{get_repo, init_git};
use crate::update::update;

use super::*;

pub fn deploy(repo_link: &str, commit_id: &str) -> Result<()> {
    if repo_link.is_empty() {
        deploy_from_local(commit_id)?;
    } else {
        deploy_from_remote(repo_link, commit_id)?;
    }

    Ok(())
}

fn deploy_from_local(commit_id: &str) -> Result<()> {
    if !commit_id.is_empty() {
        let repo = get_repo()?;

        let original_head = repo.head()?;
        let original_commit = original_head.peel_to_commit()?;

        deploy_with_commit_id(commit_id);

        create_and_copy_files();

        let mut checkout_builder = CheckoutBuilder::new();
        repo.checkout_tree(
            original_commit.tree()?.as_object(),
            Some(&mut checkout_builder),
        )?;

        repo.set_head("refs/heads/main")?;

        update(false);
    } else {
        create_and_copy_files();
    }

    Ok(())
}

fn deploy_with_commit_id(commit_id: &str) -> Result<()> {
    let repo = get_repo()?;

    let oid = Oid::from_str(commit_id)?;
    let commit = repo.find_commit(oid)?;

    let tree = commit.tree()?;
    let tree_obj = tree.as_object();

    let mut checkout_builder = CheckoutBuilder::new();
    repo.checkout_tree(tree_obj, Some(&mut checkout_builder))?;

    repo.set_head_detached(oid)?;
    Ok(())
}

fn deploy_from_remote(repo_link: &str, commit_id: &str) -> Result<()> {
    if check_for_init()? {
        empty_adof_dir()?;
    }

    let adof_dir = get_adof_dir()?;
    Repository::clone(repo_link, &adof_dir)?;

    if !commit_id.is_empty() {
        deploy_with_commit_id(commit_id);
    }

    create_and_copy_files();

    let git_dir = format!("{}/.git", adof_dir);
    fs::remove_dir_all(git_dir)?;

    init_git();
    Ok(())
}

fn empty_adof_dir() -> Result<()> {
    let adof_dir = get_adof_dir()?;
    fs::remove_dir_all(&adof_dir)?;
    Ok(())
}

fn create_and_copy_files() -> Result<()> {
    let table_struct = get_table_struct()?;

    table_struct
        .table
        .iter()
        .for_each(|(original_file, backup_file)| {
            let original_path = Path::new(original_file);

            if !original_path.exists() {
                let path_dir = original_path.parent()?;

                fs::create_dir_all(path_dir)?;
                fs::File::create(original_file)?;
            }

            fs::copy(backup_file, original_file)?;
        });

    Ok(())
}

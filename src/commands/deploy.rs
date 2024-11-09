use std::path::Path;

use git2::{build::CheckoutBuilder, Oid, Repository};

use crate::git::{add::git_add, get_repo, init_git};
use crate::unlink::unlink;

use super::*;

pub fn deploy(repo_link: &str, commit_id: &str) {
    if repo_link.is_empty() && commit_id.is_empty() {
        println!("Provide a link or commit");
        std::process::exit(1);
    } else if repo_link.is_empty() {
        deploy_with_commit_id(commit_id);
    } else {
        deploy_from_remote(repo_link, commit_id);
    }
}

fn deploy_with_commit_id(commit_id: &str) {
    let repo = get_repo();

    let oid = Oid::from_str(commit_id).unwrap();
    let commit = repo.find_commit(oid).unwrap();

    let tree = commit.tree().unwrap();
    let tree_obj = tree.as_object();

    let mut checkout_builder = CheckoutBuilder::new();
    repo.checkout_tree(tree_obj, Some(&mut checkout_builder))
        .unwrap();

    repo.set_head_detached(oid).unwrap();
}

fn deploy_from_remote(repo_link: &str, commit_id: &str) {
    let adof_dir = get_adof_dir();
    Repository::clone(repo_link, &adof_dir).unwrap();

    if !commit_id.is_empty() {
        deploy_with_commit_id(commit_id);
    }

    unlink();
    create_and_copy_files();

    let git_dir = format!("{}/.git", adof_dir);
    fs::remove_dir_all(git_dir).unwrap();

    init_git();
    git_add();
}

fn create_and_copy_files() {
    let table_struct = get_table_struct();

    table_struct
        .table
        .iter()
        .for_each(|(original_file, backup_file)| {
            let original_path = Path::new(original_file);

            if !original_path.exists() {
                let path_dir = original_path.parent().unwrap();

                fs::create_dir_all(path_dir).unwrap();
                fs::File::create(original_file).unwrap();
            }

            fs::copy(backup_file, original_file).unwrap();
        });
}

use anyhow::Result;
use chrono::Local;
use git2::Repository;

use adof::get_adof_dir;

pub mod add;
pub mod commit;
pub mod commit_message;
pub mod git_ignore;
pub mod remote;

pub fn init_git() -> Result<()> {
    let adof_dir = get_adof_dir();
    Repository::init(adof_dir)?;
    add::git_add()?;
    Ok(())
}

pub fn get_repo() -> Result<Repository> {
    let adof_dir = get_adof_dir()?;
    let repo = Repository::open(adof_dir)?;
    Ok(repo)
}

pub fn is_remote_exist() -> bool {
    get_repo().find_remote("origin").is_ok()
}

pub fn get_default_branch() -> Result<String> {
    let repo = get_repo();
    let config = repo.config()?;
    let mut default_branch = "master".to_string();

    if let Ok(branch) = config.get_string("init.defaultBranch") {
        default_branch = branch.to_string();
    }

    Ok(default_branch)
}

use anyhow::{Context, Result};
use git2::Repository;

pub mod add;
pub mod commit;
pub mod commit_message;
pub mod git_ignore;
pub mod remote;

pub fn init_git() -> Result<()> {
    let adof_dir = adof::get_adof_dir();
    Repository::init(adof_dir).context("Failed to initialize the Git repository.")?;
    add::git_add()?;
    Ok(())
}

pub fn get_repo() -> Result<Repository> {
    let adof_dir = adof::get_adof_dir();
    let repo = Repository::open(adof_dir)
        .context("Something went wrong in Git repository. Please try agin!")?;
    Ok(repo)
}

pub fn is_remote_exist() -> Result<bool> {
    let repo = get_repo()?;
    let is_remote = repo.find_remote("origin").is_ok();
    Ok(is_remote)
}

pub fn get_default_branch() -> Result<String> {
    let repo = get_repo()?;
    let config = repo.config().unwrap();

    if let Ok(default_branch) = config.get_string("init.defaultBranch") {
        Ok(default_branch.to_string())
    } else {
        Ok("master".to_string())
    }
}

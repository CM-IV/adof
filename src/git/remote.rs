use git2::{BranchType, FetchOptions, RemoteCallbacks};

use super::*;

pub fn link_remote(repo_link: &str) -> Result<()> {
    let repo = get_repo()?;

    repo.remote("origin", repo_link)
        .expect("Failed to add remote");

    let branch_name = "main";
    let mut config = repo.config().context("Failed to get config")?;

    let remote_key = format!("branch.{}.remote", branch_name);
    config
        .set_str(&remote_key, "origin")
        .context("Failed to set remote branch")?;

    let merge_key = format!("branch.{}.merge", branch_name);
    config
        .set_str(&merge_key, &format!("refs/heads/{}", branch_name))
        .context("Failed to set branch merge configuration")?;

    let mut fetch_options = FetchOptions::new();
    let mut callbacks = RemoteCallbacks::new();

    callbacks.transfer_progress(|stats| {
        println!(
            "Received {} objects in {} bytes",
            stats.received_objects(),
            stats.received_bytes()
        );
        true
    });
    fetch_options.remote_callbacks(callbacks);

    let mut remote = repo
        .find_remote("origin")
        .context("Failed to find remote")?;
    remote
        .fetch(&[branch_name], Some(&mut fetch_options), None)
        .context("Failed to fetch from remote")?;

    let origin_main_ref = repo.find_reference("refs/remotes/origin/main");
    if origin_main_ref.is_err() {
        println!("Remote repository does not have a 'main' branch.");
        std::process::exit(1);
    }

    let local_branch = match repo.find_branch(branch_name, BranchType::Local) {
        Ok(branch) => branch,
        Err(_) => {
            let commit = origin_main_ref
                .as_ref()
                .unwrap()
                .peel_to_commit()
                .context("Failed to find remote main branch commit")?;
            repo.branch(branch_name, &commit, false)
                .context("Failed to create local branch")?
        }
    };

    local_branch
        .into_reference()
        .set_target(
            origin_main_ref.unwrap().target().unwrap(),
            "Setting up tracking",
        )
        .context("Failed to set local branch to track origin")?;

    repo.set_head("refs/heads/main")
        .context("Failed to set head")?;

    Ok(())
}

pub fn unlink_remote() -> Result<()> {
    let repo = get_repo()?;
    repo.remote_delete("origin")
        .context("Failed to unlink the GitHub repository.")?;
    Ok(())
}

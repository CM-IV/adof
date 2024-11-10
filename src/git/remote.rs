use git2::{BranchType, FetchOptions, RemoteCallbacks};

use crate::push::push;

use super::get_repo;

pub fn link_remote(repo_link: &str) -> Result<()> {
    let repo = get_repo();

    repo.remote("origin", repo_link)?;

    push();

    let branch_name = "main";
    let mut config = repo.config().expect("Failed to get config");

    let remote_key = format!("branch.{}.remote", branch_name);
    config.set_str(&remote_key, "origin")?;

    let merge_key = format!("branch.{}.merge", branch_name);
    config.set_str(&merge_key, &format!("refs/heads/{}", branch_name))?;

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

    let mut remote = repo.find_remote("origin")?;

    remote.fetch(&[branch_name], Some(&mut fetch_options), None)?;

    let origin_main_ref = repo.find_reference("refs/remotes/origin/main");
    if origin_main_ref.is_err() {
        println!("Remote repository does not have a 'main' branch.");
        std::process::exit(1);
    }

    let local_branch = match repo.find_branch(branch_name, BranchType::Local) {
        Ok(branch) => branch,
        Err(_) => {
            let commit = origin_main_ref.as_ref()?.peel_to_commit()?;
            repo.branch(branch_name, &commit, false)?
        }
    };

    local_branch
        .into_reference()
        .set_target(origin_main_ref?.target().unwrap(), "Setting up tracking")
        .expect("Failed to set local branch to track origin");

    repo.set_head("refs/heads/main")
        .expect("Failed to set head");

    println!("Remote linked, branch tracking configured, and commits pushed if necessary.");
    Ok(())
}

pub fn unlink_remote() -> Result<()> {
    let repo = get_repo();
    repo.remote_delete("origin")?;
    Ok(())
}

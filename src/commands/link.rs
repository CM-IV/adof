use crate::git::get_repo;
use git2::{BranchType, FetchOptions, RemoteCallbacks};

pub fn link(repo_link: &str) {
    let repo = get_repo();

    repo.remote("origin", repo_link)
        .expect("Failed to add remote");

    let branch_name = "main";

    let mut config = repo.config().expect("Failed to get config");
    let remote_key = format!("branch.{}.remote", branch_name);
    config
        .set_str(&remote_key, "origin")
        .expect("Failed to set remote branch");
    let merge_key = format!("branch.{}.merge", branch_name);
    config
        .set_str(&merge_key, &format!("refs/heads/{}", branch_name))
        .expect("Failed to set branch merge configuration");

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

    let mut remote = repo.find_remote("origin").expect("Failed to find remote");
    remote
        .fetch(&[branch_name], Some(&mut fetch_options), None)
        .expect("Failed to fetch from remote");

    let local_branch = match repo.find_branch(branch_name, BranchType::Local) {
        Ok(branch) => branch.into_reference(),
        Err(_) => {
            let commit = repo
                .find_reference("refs/remotes/origin/main")
                .and_then(|r| r.peel_to_commit())
                .expect("Failed to find remote main branch commit");
            repo.branch(branch_name, &commit, false)
                .expect("Failed to create local branch")
                .into_reference()
        }
    };

    repo.set_head("refs/heads/main")
        .expect("Failed to set head");

    println!("Remote linked and branch tracking configured.");
}

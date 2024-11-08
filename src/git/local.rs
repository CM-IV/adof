use super::*;

pub fn get_local_commits(num: u8) -> Vec<Commit> {
    if num != 0 {
        get_local_commit(num)
    } else if !is_remote_exist() {
        get_local_commit(5)
    } else {
        let only_local_changes = get_only_local_commits();

        if only_local_changes.is_empty() {
            get_local_commit(5)
        } else {
            only_local_changes
        }
    }
}

fn get_only_local_commits() -> Vec<Commit> {
    let repo = get_repo();

    let mut remote = repo
        .find_remote("origin")
        .expect("Failed to find remote 'origin'");

    remote
        .fetch(&["refs/heads/main:refs/remotes/origin/main"], None, None)
        .unwrap_or_else(|e| {
            println!("Failed to fetch from remote: {}", e);
            std::process::exit(1);
        });

    let local_oid = repo
        .refname_to_id("refs/heads/main")
        .expect("Local branch 'main' not found");

    let remote_oid = match repo.refname_to_id("refs/remotes/origin/main") {
        Ok(oid) => oid,
        Err(_) => {
            println!("Remote branch 'origin/main' not found after fetch.");
            return vec![];
        }
    };

    let local_commit = repo
        .find_commit(local_oid)
        .expect("Failed to find local commit");
    let remote_commit = repo
        .find_commit(remote_oid)
        .expect("Failed to find remote commit");

    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk
        .push(local_commit.id())
        .expect("Failed to push local commit");
    revwalk
        .hide(remote_commit.id())
        .expect("Failed to hide remote commit");

    let mut commits = Vec::new();
    for commit_id in revwalk {
        let commit = repo
            .find_commit(commit_id.unwrap())
            .expect("Failed to find commit");
        commits.push(Commit::new(
            &commit.id().to_string(),
            commit.message().unwrap(),
        ));
    }

    commits
}

pub fn get_remote_behind_commits_no() -> usize {
    get_only_local_commits().len()
}

fn get_local_commit(num: u8) -> Vec<Commit> {
    let repo = get_repo();
    let head = repo.head().unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push(head.target().unwrap()).unwrap();

    let mut commits = Vec::new();

    for oid in revwalk {
        let oid = oid.unwrap();

        let commit = repo.find_commit(oid).unwrap();
        let commit = Commit::new(&commit.id().to_string(), commit.message().unwrap());
        commits.push(commit);

        if commits.len() == num as usize {
            break;
        }
    }

    commits
}

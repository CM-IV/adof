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

    let local_branch = repo.head().unwrap().peel_to_commit().unwrap();

    let remote_name = "origin";
    let branch_name = "main";
    let remote_branch = repo
        .find_branch(
            &format!("{}/{}", remote_name, branch_name),
            git2::BranchType::Remote,
        )
        .unwrap();
    let remote_commit = remote_branch.get().peel_to_commit().unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push(local_branch.id()).unwrap();
    revwalk.hide(remote_commit.id()).unwrap();

    let mut commits = Vec::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid.unwrap()).unwrap();

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

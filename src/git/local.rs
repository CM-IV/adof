use super::*;

pub fn get_local_changes(num: u8) -> Vec<Commit> {
    if num == 0 {
        let local_commits = get_only_local_commits();

        if local_commits.is_empty() {
            get_local_commits(5)
        } else {
            local_commits
        }
    } else {
        get_local_commits(num)
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

fn get_local_commits(num: u8) -> Vec<Commit> {
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

        if commits.len() > num.into() {
            break;
        }
    }

    commits
}

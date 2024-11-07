use super::*;

pub fn get_local_changes() -> Vec<Commit> {
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

        if commits.len() > 255 {
            break;
        }
    }

    commits
}

use super::*;

pub fn get_remote_commits(mut num: u8) -> Vec<Commit> {
    if !is_remote_exist() {
        println!("No remote branch is connected");
        std::process::exit(1);
    }

    if num == 0 {
        num = 5;
    }

    let repo = get_repo();

    let mut remote = repo.find_remote("origin").unwrap();
    remote.fetch(&["main"], None, None).unwrap();

    let remote_branch_name = "refs/remotes/origin/main";
    let remote_branch = repo.find_reference(remote_branch_name).unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push(remote_branch.target().unwrap()).unwrap();

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

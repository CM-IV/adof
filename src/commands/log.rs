use crate::git::local::get_local_changes;

pub fn log() {
    let local_commits = get_local_changes();

    for commit in local_commits {
        println!("Commit ID: {:?}, Commit message: {:?}", commit.id, commit.message);
    }
}

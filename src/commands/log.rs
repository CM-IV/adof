use crate::git::{local::get_local_commits, Commit};

pub fn log(num: u8) {
    let commits_to_display = get_local_commits(num);
    display_commits(&commits_to_display);
}

fn display_commits(commits: &Vec<Commit>) {
    for commit in commits {
        println!(
            "Commit id: {:?}, Commit message: {:?}",
            commit.id, commit.message
        );
    }
}

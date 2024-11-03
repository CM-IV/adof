use crate::git::get_repo;

pub fn link(repo_link: &str) {
    let repo = get_repo();
    repo.remote_set_url("origin", repo_link).unwrap();
}

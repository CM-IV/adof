use crate::git::get_repo;

pub fn unlink() {
    let repo = get_repo();
    repo.remote_delete("origin").unwrap();
}

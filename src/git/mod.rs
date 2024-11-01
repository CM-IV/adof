use git2::Repository;

use adof::get_adof_dir;

pub mod add;
pub mod commit;
pub mod branch;
pub mod merge;

pub fn init_git() {
    let adof_dir = get_adof_dir();
    Repository::init(adof_dir).unwrap();
    add::add()
}

fn get_repo() -> Repository {
    let adof_dir = get_adof_dir();
    let repo = Repository::open(adof_dir).unwrap();
    repo
}

use git2::Repository;

use adof::get_adof_dir;

pub mod add;
pub mod branch;
pub mod commit;
pub mod merge;

pub fn init_git() {
    let adof_dir = get_adof_dir();
    Repository::init(adof_dir).unwrap();
    add::add()
}

fn get_repo() -> Repository {
    let adof_dir = get_adof_dir();
    Repository::open(adof_dir).unwrap()
}

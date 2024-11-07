use crate::git::local::get_local_changes;

pub fn log() {
    get_local_changes();
}

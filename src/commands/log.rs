pub fn log(num: u8, remote: bool) {
    if remote {
        show_remote_commits(num);
    } else {
        if num == 0 {
            show_only_local_commits();
        } else {
            show_local_commits(num);
        }
    }
}

fn show_local_commits(num: u8) {}

fn show_remote_commits(num: u8) {}

fn show_only_local_commits() {}

fn get_only_local_commits_no() -> u8 {5}

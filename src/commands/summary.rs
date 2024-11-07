pub fn summary() {
    println!("Get the summary");
}

fn get_last_local_commit() -> Vec<Commit> {
    get_local_commits(1)
}

fn get_last_remote_commit() -> Vec<Commit> {
    if !is_remote_exist() {
        println!("Connect to remote first");
    }

    get_remote_commits(1)
}

fn get_only_local_commits_no() -> usize {
    get_remote_behind_commits_no()
}

fn get_files_being_tracked_no() -> usize {
    let table_struct = get_table_struct();
    table_struct.table.len()
}

fn is_auto_update_enabled() -> bool {
    let mut auto_update = false;

    let pid_file = format!("{}/do_not_touch/pid.txt", get_adof_dir());

    auto_update
}

fn check_for_update() -> Vec<String> {}

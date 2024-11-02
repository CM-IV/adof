use std::fs;
use std::path::PathBuf;

use glob::glob;

use adof::{get_adof_dir, get_home_dir};

use crate::{database::add, git::init_git};

use super::*;

const FILE_PATTERNS: [&str; 17] = [
    ".bashrc",
    ".zshrc",
    ".bash_profile",
    ".profile",
    ".vimrc",
    ".gitconfig",
    ".config/nvim/**/*.vim",
    ".config/git/config",
    ".cargo/config",
    ".config/starship.toml",
    ".config/tmux/tmux.conf",
    ".config/alacritty/alacritty.yml",
    ".docker/config.json",
    ".npmrc",
    ".yarnrc",
    ".config/wezterm/wezterm.lua",
    ".config/nvim/**/*.lua",
];

pub fn init() {
    let found_files = find_files();
    let selected_files = select_files(found_files);

    create_adof_dir();

    create_backup_files(&selected_files);
    init_git();
}

fn create_adof_dir() {
    let adof_dir = get_adof_dir();
    fs::create_dir_all(&adof_dir).expect("failed to create adof dir.");
}

fn find_files() -> Vec<PathBuf> {
    let home_dir = get_home_dir();

    let mut found_files = Vec::new();

    for pattern in FILE_PATTERNS {
        let pattern_path = format!("{}/{}", home_dir, pattern);

        for entry in glob(&pattern_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => found_files.push(path),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    found_files
}

fn create_backup_files(selected_files: &[String]) {
    (0..selected_files.len()).for_each(|i| {
        let backup_file = create_backup_file(&selected_files[i]);
        fs::copy(&selected_files[i], &backup_file).unwrap();
        add::add_files_to_database(&selected_files[i], &backup_file);
    })
}

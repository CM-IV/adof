use std::fs;
use std::io::Write;
use std::path::{self, PathBuf};
use std::process::{Command, Stdio};

use glob::glob;

use adof::{get_adof_dir, get_home_dir};
use crate::{database::add, git::init_git};

const FILE_PATTERN: [&str; 17] = [
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

    create_copy_selected_files(&selected_files);
    init_git();
}

fn create_adof_dir() {
    let adof_dir = get_adof_dir();
    fs::create_dir_all(&adof_dir).expect("failed to create adof dir.");
}

fn find_files() -> Vec<PathBuf> {
    let home_dir = get_home_dir();

    let mut found_files = Vec::new();

    for pattern in FILE_PATTERN {
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

fn select_files(found_files: Vec<PathBuf>) -> Vec<String> {
    let files_input = found_files
        .iter()
        .map(|file| file.clone().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut child = Command::new("fzf")
        .arg("--preview")
        .arg("cat {}")
        .arg("-m")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fzf");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(files_input.as_bytes())
            .expect("Failed to write to fzf stdin");
    }

    let output = child.wait_with_output().expect("Failed to read fzf output");

    let selected_files = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
        .lines()
        .map(|file| file.to_string())
        .collect::<Vec<String>>();

    if selected_files.is_empty() {
        println!("No file selected.");
    }

    selected_files
}

fn create_file(file: &str) {
    let path = path::Path::new(file);
    let path_dir = path.parent().unwrap();
    fs::create_dir_all(path_dir).unwrap();
    fs::File::create(file).unwrap();
}

fn create_copy_selected_files(selected_files: &Vec<String>) {
    let home_dir = get_home_dir();
    let adof_dir = get_adof_dir();

    let files_to_create = selected_files
        .iter()
        .map(|file| file.replace(&home_dir, &adof_dir))
        .collect::<Vec<String>>();

    (0..files_to_create.len()).for_each(|i| {
        create_file(&files_to_create[i]);
        fs::copy(&selected_files[i], &files_to_create[i]).unwrap();
        add::add_files_to_database(&selected_files[i], &files_to_create[i]);
    })
}

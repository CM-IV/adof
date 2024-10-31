use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use glob::glob;

pub fn init() {
    let found_dotfiles = find_dotfiles();
    let selected_dotfiles = show_them_in_fzf(found_dotfiles);
    let adof_dir_path = create_adof_dir();
    create_selected_dotfiles(&selected_dotfiles, &adof_dir_path);
}

fn find_dotfiles() -> Vec<PathBuf> {
    let home_dir = env::var("HOME").expect("Could not retrieve home directory");

    let dotfiles_patterns = vec![
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

    let mut found_files = Vec::new();

    for pattern in dotfiles_patterns {
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

fn show_them_in_fzf(found_dotfiles: Vec<PathBuf>) -> Vec<String> {
    let files_input = found_dotfiles
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

fn create_adof_dir() -> String {
    let home_dir = env::var("HOME").expect("Failed to retrieve home dir.");
    let adof_dir = format!("{}/{}", home_dir, ".adof");
    std::fs::create_dir_all(&adof_dir).expect("failed to create adof dir.");
    adof_dir
}

fn create_selected_dotfiles(selected_dotfiles: &Vec<String>, adof_dir_path: &str) {
    let home_dir = env::var("HOME")
        .expect("Failed to retrieve home dir.")
        .to_string();

    let files_to_create = selected_dotfiles
        .iter()
        .map(|file| file.replace(&home_dir, adof_dir_path))
        .collect::<Vec<String>>();

    files_to_create.iter().for_each(|file| {
        std::fs::File::create(file).unwrap();
    });

    (0..files_to_create.len()).for_each(|i| {
        std::fs::copy(&selected_dotfiles[i], &files_to_create[i]).unwrap();
    })
}

// fn establish_symlinks() {}

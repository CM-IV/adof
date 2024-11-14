use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;

use glob::glob;

use adof::get_home_dir;

use crate::database::add::add_files;
use crate::git::add::git_add;

use super::*;

pub async fn add() -> Result<()> {
    if !check_for_init()? {
        eprintln!("Adof is not initialized.");
        std::process::exit(1);
    }

    let files_to_add = get_files_to_add().context("Failed to retrieve files to add")?;

    if files_to_add.is_empty() {
        println!("Please select new files.");
        process::exit(1);
    }

    create_backup_files(&files_to_add).context("Failed to create backup files")?;
    git_add().context("Failed to add files to git")?;

    println!("Files added successfully.");

    Ok(())
}

fn get_files_to_add() -> Result<Vec<String>> {
    let home_dir = get_home_dir();
    let pattern = format!("{}/**/*", home_dir);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) if path.is_file() => tx.send(path).expect("Failed to send file path"),
                Err(e) => eprintln!("Glob error: {:?}", e),
                _ => (),
            }
        }
    });

    let selected_files = select_files(rx).context("Failed to select files using fzf")?;

    Ok(selected_files
        .into_iter()
        .filter(|file| !is_file_backedup(file).unwrap_or(false))
        .collect())
}

fn select_files(rx: Receiver<PathBuf>) -> Result<Vec<String>> {
    let mut child = Command::new("fzf")
        .arg("--preview")
        .arg("cat {}")
        .arg("-m")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to start fzf command")?;

    let mut stdin = child.stdin.take().context("Failed to open fzf stdin")?;

    thread::spawn(move || {
        for path in rx.iter() {
            if let Some(file_path) = path.to_str() {
                if writeln!(stdin, "{}", file_path).is_err() {
                    eprintln!("Failed to write file path to fzf stdin");
                }
            }
        }
    });

    let output = child
        .wait_with_output()
        .context("Failed to read fzf output")?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .lines()
        .map(|file| file.to_string())
        .collect())
}

fn create_backup_files(files_to_add: &[String]) -> Result<()> {
    files_to_add.iter().try_for_each(|file_path| {
        let backup_file = create_file(file_path).context("Failed to create backup file")?;
        fs::copy(file_path, &backup_file).context("Failed to copy file to backup location")?;
        add_files(file_path, &backup_file)?;
        Ok(())
    })
}

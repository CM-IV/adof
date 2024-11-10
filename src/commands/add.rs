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
use crate::init::init;

use super::*;

pub async fn add() -> Result<()> {
    if !check_for_init()? {
        init().await?;
        process::exit(1);
    }

    let files_to_add = get_files_to_add()?;

    if files_to_add.is_empty() {
        println!("files are already exist");
        process::exit(1);
    }

    create_backup_files(&files_to_add)?;
    git_add();

    Ok(())
}

fn get_files_to_add() -> Result<Vec<String>> {
    let home_dir = get_home_dir()?;
    let pattern = format!("{}/**/*", home_dir);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for entry in glob(&pattern)? {
            let path = entry?;

            if path.is_file() {
                tx.send(path)?;
            }
        }
    });

    let selected_files = select_files(rx);

    let files_to_add = selected_files
        .into_iter()
        .filter(|file| !is_file_backedup(file))
        .collect::<Vec<String>>()?;

    Ok(files_to_add)
}

fn select_files(rx: Receiver<PathBuf>) -> Result<Vec<String>> {
    let mut child = Command::new("fzf")
        .arg("--preview")
        .arg("cat {}")
        .arg("-m")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take()?;

    thread::spawn(move || {
        for path in rx.iter() {
            if let Some(file_path) = path.to_str() {
                writeln!(stdin, "{}", file_path)?;
            }
        }
    });

    let output = child.wait_with_output()?;

    let selected_files = String::from_utf8_lossy(&output.stdout)
        .trim()
        .lines()
        .map(|file| file.to_string())
        .collect::<Vec<String>>();

    Ok(selected_files)
}

fn create_backup_files(files_to_add: &[String]) -> Result<()> {
    (0..files_to_add.len()).for_each(|i| {
        let backup_file = create_file(&files_to_add[i])?;
        fs::copy(&files_to_add[i], &backup_file)?;
        add_files(&files_to_add[i], &backup_file)?;
    });

    Ok(())
}

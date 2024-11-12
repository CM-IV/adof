use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::{
    fs,
    path::{self, PathBuf},
};

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

use crate::database::{get_database_path, get_table_struct};

pub mod add;
pub mod auto_update;
pub mod deploy;
pub mod init;
pub mod link;
pub mod list;
pub mod log;
pub mod patterns;
pub mod push;
pub mod readme;
pub mod remove;
pub mod summary;
pub mod uninstall;
pub mod unlink;
pub mod update;

fn select_files(found_files: Vec<PathBuf>) -> Result<Vec<String>> {
    let found_files = found_files
        .iter()
        .map(|file| {
            file.clone()
                .into_os_string()
                .into_string()
                .map_err(|os_str| {
                    anyhow::anyhow!("Failed to convert OsString to String: {:?}", os_str)
                })
        })
        .collect::<Result<Vec<String>>>()?
        .join("\n");

    let mut child = Command::new("fzf")
        .arg("--preview")
        .arg("cat {}")
        .arg("-m")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to start fzf")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(found_files.as_bytes())
            .context("Failed to write to fzf stdin")?;
    }

    let output = child
        .wait_with_output()
        .context("Failed to read fzf output")?;

    let selected_files = String::from_utf8_lossy(&output.stdout)
        .trim()
        .lines()
        .map(|file| file.to_string())
        .collect::<Vec<String>>();

    if selected_files.is_empty() {
        println!("No file selected.");
    }

    Ok(selected_files)
}

pub fn create_file(original_file: &str) -> Result<String> {
    let home_dir = adof::get_home_dir();
    let adof_dir = adof::get_adof_dir();

    let backup_file = original_file.replace(&home_dir, &adof_dir);

    let path = path::Path::new(&backup_file);
    let path_dir = path.parent().context("Failed to get parent directory")?;

    fs::create_dir_all(path_dir).context("Failed to create backup directory")?;
    fs::File::create(&backup_file).context("Failed to create backup file")?;

    Ok(backup_file)
}

fn calculate_file_hash(file_path: &str) -> Result<Vec<u8>> {
    let mut file = fs::File::open(file_path).context(format!(
        "Failed to open file for hash calculation: {}",
        file_path
    ))?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .context("Failed to read file into buffer")?;
    hasher.update(&buffer);
    Ok(hasher.finalize().to_vec())
}

fn is_file_backedup(original_file: &str) -> Result<bool> {
    let table_struct = get_table_struct()?;
    Ok(table_struct.table.contains_key(original_file))
}

fn check_for_init() -> Result<bool> {
    let database_path = get_database_path();
    fs::exists(&database_path).context("Failed to check if the adof is already initialized")
}

fn get_pid_file() -> String {
    let adof_dir = adof::get_adof_dir();
    format!("{}/do_not_touch/pid.txt", adof_dir)
}

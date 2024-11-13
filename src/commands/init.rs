use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process;

use glob::glob;

use adof::get_home_dir;

use crate::commands::patterns::FILE_PATTERNS;
use crate::commands::readme::create_readme;
use crate::{
    database::{add, create::create_database},
    git::{git_ignore::create_git_ignore, init_git},
};

use super::*;

pub async fn init() -> Result<()> {
    if check_for_init()? {
        println!("Already initialized");
        process::exit(1);
    }

    create_database().context("Failed to create database")?;
    create_git_ignore().context("Failed to create .gitignore")?;

    let readme_task = tokio::spawn(async { create_readme().await });
    let found_files_task = tokio::spawn(async { find_files() });

    readme_task.await?.context("Failed to create README")?;
    let found_files = found_files_task.await.context("Failed to find files")??;

    let selected_files = select_files(found_files).context("Failed to select files")?;

    create_backup_files(&selected_files).context("Failed to create backup files")?;
    init_git().context("Failed to initialize git")?;

    println!("Adof has been initialized.");
    Ok(())
}

fn find_files() -> Result<Vec<PathBuf>> {
    let home_dir = get_home_dir();

    let mut found_files = Vec::new();

    for pattern in FILE_PATTERNS {
        let pattern_path = format!("{}/{}", home_dir, pattern);

        for entry in glob(&pattern_path).context("Failed to read glob pattern")? {
            match entry {
                Ok(path) => found_files.push(path),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    Ok(found_files)
}

fn create_backup_files(selected_files: &[String]) -> Result<()> {
    for file in selected_files {
        let backup_file = create_file(file).context("Failed to create backup file")?;
        fs::copy(file, &backup_file)
            .with_context(|| format!("Failed to copy file {} to backup {}", file, backup_file))?;
        add::add_files(file, &backup_file).context("Failed to add file to database")?;
    }
    Ok(())
}

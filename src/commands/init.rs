use std::fs;
use std::path::PathBuf;
use std::process;

use glob::glob;

use adof::get_home_dir;

use crate::commands::patterns::FILE_PATTERNS;
use crate::commands::readme::create_readme;
use crate::{
    database::{add, create_database},
    git::{git_ignore::create_git_ignore, init_git},
};

use super::*;

pub async fn init() -> Result<()> {
    if check_for_init() {
        println!("Already initialized");
        process::exit(1);
    }

    create_database();
    create_git_ignore();

    let readme_task = tokio::spawn(async { create_readme().await });
    let found_files_task = tokio::spawn(async { find_files() });

    readme_task.await?;
    let found_files = found_files_task.await?;

    let selected_files = select_files(found_files);

    create_backup_files(&selected_files);
    init_git();

    Ok(())
}

fn find_files() -> Result<Vec<PathBuf>> {
    let home_dir = get_home_dir()?;

    let mut found_files = Vec::new();

    for pattern in FILE_PATTERNS {
        let pattern_path = format!("{}/{}", home_dir, pattern);

        for entry in glob(&pattern_path)? {
            found_files.push(entry?);
        }
    }

    Ok(found_files)
}

fn create_backup_files(selected_files: &[String]) -> Result<()> {
    (0..selected_files.len()).for_each(|i| {
        let backup_file = create_file(&selected_files[i])?;
        fs::copy(&selected_files[i], &backup_file)?;
        add::add_files(&selected_files[i], &backup_file);
    });

    Ok(())
}

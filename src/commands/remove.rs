use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::*;
use crate::database::remove::remove_files;
use crate::git::add::git_add;

pub fn remove() -> Result<()> {
    let files_to_remove = get_files_to_remove()?;

    if files_to_remove.is_empty() {
        println!("Nothing selected.");
        std::process::exit(0);
    }

    remove_selected_files(&files_to_remove)?;
    git_add()?;

    println!("Files are removed successfully.");
    Ok(())
}

fn get_files_to_remove() -> Result<Vec<String>> {
    let mut found_files = Vec::new();

    let table_struct = get_table_struct()?;
    table_struct.table.values().for_each(|backedup_file| {
        found_files.push(backedup_file.into());
    });

    select_files(found_files)
}

fn remove_selected_files(files_to_remove: &[String]) -> Result<()> {
    files_to_remove.iter().try_for_each(|file| {
        remove_files(file)?;
        remove_dir(file)
    })
}

fn remove_dir(file: &str) -> Result<()> {
    if let Some(dir) = Path::new(file).parent() {
        if is_dir_empty(dir) {
            fs::remove_dir(dir).context(format!("Failed to remove empty directory {:?}", dir))?;
            remove_dir(&dir.display().to_string())?;
        }
    }
    Ok(())
}

fn is_dir_empty(dir: &Path) -> bool {
    match fs::read_dir(dir) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}

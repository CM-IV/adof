use std::path::Path;

use crate::database::remove::remove_files;
use crate::git::add::git_add;

use super::*;

pub fn remove() -> Result<()> {
    let files_to_remove = get_files_to_remove()?;
    remove_selected_files(&files_to_remove)?;
    git_add();
    Ok(())
}

fn get_files_to_remove() ->Result<Vec<String>> {
    let mut found_files = Vec::new();

    let table_struct = get_table_struct()?;
    table_struct.table.values().for_each(|backedup_file| {
        found_files.push(backedup_file.into());
    });

    select_files(found_files)
}

fn remove_selected_files(files_to_remove: &[String]) -> Result<()> {
    files_to_remove.iter().for_each(|file| {
        remove_files(file);
        remove_dir(file)?;
    });

    Ok(())
}

fn remove_dir(file: &str) -> Result<()> {
    if let Some(dir) = Path::new(file).parent() {
        if is_dir_empty(dir) {
            fs::remove_dir(dir)?;
            remove_dir(&dir.display().to_string());
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

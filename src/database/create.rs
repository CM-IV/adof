use std::path::Path;

use super::*;

pub fn create_database() -> Result<()> {
    let database_path = get_database_path();
    let database_dir = Path::new(&database_path)
        .parent()
        .with_context(|| format!("Failed to create the database at {:?}.", database_path))?;
    fs::create_dir_all(database_dir)
        .with_context(|| format!("Failed to create the database at {:?}.", database_path))?;

    fs::File::create(&database_path)
        .with_context(|| format!("Failed to create the database at {:?}.", database_path))?;

    let table_struct = DataTable::new();
    let json_table = serde_json::to_string_pretty(&table_struct)
        .context("Something went wrong. Please try again.")?;

    fs::write(&database_path, json_table)
        .with_context(|| format!("Failed to create the database at {:?}.", database_path))?;
    Ok(())
}

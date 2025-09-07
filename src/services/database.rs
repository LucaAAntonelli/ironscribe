use rusqlite::{self, Connection};
use std::path::PathBuf;

pub fn create_db(path: PathBuf) -> anyhow::Result<()> {
    // Create SQLite file
    let filename = "library.db";
    let full_path = path.join(filename);
    std::fs::File::create(full_path.clone())?;

    // Initialize with schema
    let db = Connection::open(full_path)?;
    let sql = std::fs::read_to_string("schema.sql").expect("Failed to parse SQL schema file!");
    db.execute(&sql, [])?;

    Ok(())
}

#![cfg(feature = "server")]
use crate::backend::database::{DB, DB_PATH};
use anyhow::{anyhow, Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;

use crate::shared::types::Config;

pub fn create_db(path: PathBuf) -> Result<()> {
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

pub fn set_db_path(new_path: PathBuf) -> Result<()> {
    if let Some(parent) = new_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    {
        let conn =
            Connection::open(&new_path).with_context(|| format!("Opening DB at {new_path:?}"))?;
        run_migrations(&conn)?;
    }

    DB_PATH
        .set(new_path.clone())
        .map_err(|_| anyhow!("DB path already set!"))?;

    let mut cfg = Config::read()?;
    cfg.data_dir = Some(new_path);
    cfg.write()?;
    Ok(())
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    let sql = include_str!("./schema.sql");
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    conn.execute_batch(sql)?;
    Ok(())
}

pub fn is_db_configured() -> bool {
    DB_PATH.get().is_some()
}

pub fn with_conn<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&Connection) -> Result<R>,
{
    DB.with(|cell| -> Result<R> {
        if cell.borrow().is_none() {
            let path = DB_PATH
                .get()
                .ok_or_else(|| anyhow!("DB path not configured yet!"))?
                .clone();
            let conn = Connection::open(path)?;
            conn.execute("PRAGMA foreign_keys = ON", [])?;
            run_migrations(&conn)?;
        }
        let conn = cell.borrow();
        let conn = conn.as_ref().unwrap();
        f(conn)
    })
}

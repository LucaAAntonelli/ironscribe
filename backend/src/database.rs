use crate::config::Config;
use anyhow::{anyhow, Context, Result};
use once_cell::sync::OnceCell;
use rusqlite::Connection;
use std::cell::RefCell;
use std::path::PathBuf;

pub static DB_PATH: OnceCell<PathBuf> = OnceCell::new();

thread_local! {
    pub static DB: RefCell<Option<rusqlite::Connection>> = const { RefCell::new(None) };
}

// Provide a directory path; we ensure a file named "library.db" exists/initialized within it.
pub fn set_db_path(input_path: PathBuf) -> Result<()> {
    tracing::info!("set_db_path called with input: {:?}", input_path);

    // Error out if empty path provided
    if input_path.as_os_str().is_empty() {
        return Err(anyhow!("Provided path is empty"));
    }

    // Accept legacy usage where a full file path (ending in .db) was stored previously.
    let dir_path = if input_path.extension().map(|e| e == "db").unwrap_or(false) {
        input_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(input_path.clone())
    } else if input_path.is_file() {
        input_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(input_path.clone())
    } else {
        input_path.clone()
    };

    // Ensure directory exists (create full tree). If a non-directory exists at that path -> error.
    if dir_path.exists() && !dir_path.is_dir() {
        return Err(anyhow!(
            "Provided path exists but is not a directory: {dir_path:?}"
        ));
    }
    std::fs::create_dir_all(&dir_path)
        .with_context(|| format!("Creating/ensuring data directory {dir_path:?}"))?;

    let db_file_path = dir_path.join("library.db");
    let first_time = !db_file_path.exists();
    tracing::debug!(
        "Resolved data directory: {:?}; db file: {:?}",
        dir_path,
        db_file_path
    );

    // Handle already-initialized global path.
    if let Some(existing) = DB_PATH.get() {
        if *existing != db_file_path {
            tracing::warn!(
                "Attempt to change DB path at runtime from {:?} to {:?}. This is currently unsupported for safety; restart the application to switch.",
                existing, db_file_path
            );
            return Err(anyhow!(
                "Database path already set; restart application to change it."
            ));
        } else {
            tracing::debug!(
                "DB path already set to requested path; reusing connection/migrations."
            );
        }
    }

    let conn = Connection::open(&db_file_path)
        .with_context(|| format!("Opening DB file at {db_file_path:?}"))?;
    run_migrations(&conn)?; // idempotent

    // Only set OnceCell if not already set
    if DB_PATH.get().is_none() {
        DB_PATH
            .set(db_file_path.clone())
            .map_err(|_| anyhow!("DB path already set!"))?;
    }

    // Store connection in thread-local for reuse in this thread
    DB.with(|cell| {
        *cell.borrow_mut() = Some(conn);
    });

    // Persist the directory (not the file path) in config (overwrite if legacy file path was there)
    let mut cfg = Config::read()?;
    if cfg.data_dir.as_ref() != Some(&dir_path) {
        cfg.data_dir = Some(dir_path.clone());
        cfg.write()?;
    }

    if first_time {
        tracing::info!("Initialized new SQLite database at {:?}", db_file_path);
    } else {
        tracing::info!("Opened existing SQLite database at {:?}", db_file_path);
    }
    Ok(())
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    let sql = include_str!("./schema.sql");
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    conn.execute_batch(sql)?;
    Ok(())
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
            *cell.borrow_mut() = Some(conn);
        }
        let conn_ref = cell.borrow();
        let conn = conn_ref.as_ref().unwrap();
        f(conn)
    })
}

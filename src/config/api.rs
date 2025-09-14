#![cfg(feature = "server")]
use crate::config::{persist_config, Config};
use crate::database::connection::set_db_path; // temporary until refactor complete
use dioxus::prelude::*;
use std::path::PathBuf;

#[server]
pub async fn create_config() -> Result<(), ServerFnError> {
    match Config::file_exists() {
        Ok(exists) => {
            if exists { tracing::info!("Config exists, skipping creation"); Ok(()) } else {
                tracing::info!("Creating config file");
                Config::init().map_err(ServerFnError::new)?; Ok(())
            }
        }
        Err(e) => Err(ServerFnError::new(format!("Failed to check file system: {e}"))),
    }
}

#[server]
pub async fn read_config() -> Result<Config, ServerFnError> {
    create_config().await?;
    let mut cfg = Config::read().map_err(ServerFnError::new)?;
    if let Some(dir) = cfg.data_dir.clone() {
        if crate::database::connection::DB_PATH.get().is_none() {
            if let Err(e) = set_db_path(dir.clone()) {
                tracing::error!("Failed to initialize DB using configured directory {:?}: {e}", dir);
                cfg.data_dir = None;
            }
        }
    }
    Ok(cfg)
}

#[server]
pub async fn write_config(config: Config) -> Result<(), ServerFnError> {
    config.write().map_err(ServerFnError::new)
}

#[server]
pub async fn write_path(path: PathBuf) -> Result<(), ServerFnError> {
    set_db_path(path).map_err(ServerFnError::new)?; Ok(())
}

#[server]
pub async fn persist_path(path: PathBuf) -> Result<(), ServerFnError> {
    persist_config(Config { data_dir: Some(path) }).map_err(ServerFnError::new)
}

use dioxus::prelude::*;
use serverlib::config::persist_config;
use serverlib::config::Config;
use serverlib::database::set_db_path;
use std::path::PathBuf;

#[server]
pub async fn create_config() -> Result<(), ServerFnError> {
    match Config::file_exists() {
        Ok(exists) => {
            if exists {
                // nothing to do, exists already
                tracing::info!("Config exists, skipping creation");
                Ok(())
            } else {
                // create config file
                tracing::info!("Creating config file");
                Config::init().map_err(ServerFnError::new)?;
                Ok(())
            }
        }
        Err(e) => Err(ServerFnError::new(format!(
            "Failed to check file system: {e}"
        ))),
    }
}

#[server]
pub async fn read_config() -> Result<Config, ServerFnError> {
    // Ensure config file exists (noop if it already does)
    create_config().await?;
    let mut cfg = Config::read().map_err(ServerFnError::new)?;

    // If a path (directory) is configured, validate it exists (or can be created) and open/create DB inside.
    if let Some(dir) = cfg.data_dir.clone() {
        // Always attempt to set path (handles both existing and non-existing directories). It will
        // create the directory tree if needed.
        if serverlib::database::DB_PATH.get().is_none() {
            if let Err(e) = set_db_path(dir.clone()) {
                tracing::error!(
                    "Failed to initialize DB using configured directory {:?}: {e}",
                    dir
                );
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
    // User supplies a path to a directory. Create/open library.db inside & run migrations.
    set_db_path(path).map_err(ServerFnError::new)?;
    Ok(())
}

#[server]
pub async fn persist_path(path: PathBuf) -> Result<(), ServerFnError> {
    persist_config(Config {
        data_dir: Some(path),
    })
    .map_err(ServerFnError::new)
}

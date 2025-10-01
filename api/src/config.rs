#[cfg(feature = "backend")]
use backend::config::ConfigInterface;
use dioxus::prelude::*;
use shared::types::AppConfig;
use std::path::PathBuf;

#[server]
pub async fn initialize_config() -> Result<AppConfig, ServerFnError> {
    let config = AppConfig::new()?;
    Ok(config)
}

#[server]
pub async fn read_config() -> Result<AppConfig, ServerFnError> {
    let config = AppConfig::read()?;
    Ok(config)
}

#[server]
pub async fn write_config(config: AppConfig) -> Result<(), ServerFnError> {
    config.write()
}

#[server]
pub async fn create_config() -> Result<(), ServerFnError> {
    match backend::config::Config::file_exists() {
        Ok(exists) => {
            if exists {
                // nothing to do, exists already
                tracing::info!("Config exists, skipping creation");
                Ok(())
            } else {
                // create config file
                tracing::info!("Creating config file");
                backend::config::Config::init().map_err(ServerFnError::new)?;
                Ok(())
            }
        }
        Err(e) => Err(ServerFnError::new(format!(
            "Failed to check file system: {e}"
        ))),
    }
}

#[server]
pub async fn read_config() -> Result<AppConfig, ServerFnError> {
    create_config().await?;
    let mut cfg = backend::config::Config::read().map_err(ServerFnError::new)?;

    // If a path (directory) is configured, validate it exists (or can be created) and open/create DB inside.
    if let Some(dir) = cfg.data_dir.clone() {
        // Always attempt to set path (handles both existing and non-existing directories). It will
        // create the directory tree if needed.
        if backend::database::DB_PATH.get().is_none() {
            if let Err(e) = backend::database::set_db_path(dir.clone()) {
                tracing::error!(
                    "Failed to initialize DB using configured directory {:?}: {e}",
                    dir
                );
                cfg.data_dir = None;
            }
        }
    }
    Ok(AppConfig::from(cfg))
}

#[server]
pub async fn write_path(path: PathBuf) -> Result<(), ServerFnError> {
    backend::database::set_db_path(path).map_err(ServerFnError::new)?;
    Ok(())
}

// Server wrapper around the backend::config::init_config function which was previously
// invoked directly from the web crate's main() function. This ensures initialization
// happens through a Dioxus server function boundary so the client can trigger it and
// we avoid direct backend calls in the web entrypoint.
#[server]
pub async fn init_config_server() -> Result<(), ServerFnError> {
    backend::config::init_config()
        .map(|_| ())
        .map_err(ServerFnError::new)?;
    Ok(())
}

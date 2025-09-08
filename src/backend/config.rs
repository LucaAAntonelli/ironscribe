#[cfg(feature = "server")]
use crate::services::config::persist_config;
use crate::shared::types::Config;
use dioxus::prelude::*;
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
    // Call create function, will do nothing if file exists already
    create_config().await?;
    Config::read().map_err(ServerFnError::new)
}

#[server]
pub async fn write_config(config: Config) -> Result<(), ServerFnError> {
    config.write().map_err(ServerFnError::new)
}

#[server]
pub async fn write_path(path: PathBuf) -> Result<(), ServerFnError> {
    let config = Config {
        data_dir: Some(path),
    };
    write_config(config).await
}

#[server]
pub async fn persist_path(path: PathBuf) -> Result<(), ServerFnError> {
    persist_config(Config {
        data_dir: Some(path),
    })
    .map_err(ServerFnError::new)
}

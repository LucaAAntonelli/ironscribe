use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Lightweight config representation usable on the client (wasm) without depending
// on the backend crate (which pulls in rusqlite and native libs).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub data_dir: Option<PathBuf>,
}

// TODO: get rid of duplicate AppConfig struct and just use Config everywhere
#[cfg(feature = "server")]
impl From<backend::config::Config> for AppConfig {
    fn from(c: backend::config::Config) -> Self {
        Self {
            data_dir: c.data_dir,
        }
    }
}

#[cfg(feature = "server")]
impl From<AppConfig> for backend::config::Config {
    fn from(c: AppConfig) -> Self {
        backend::config::Config {
            data_dir: c.data_dir,
        }
    }
}

#[server]
pub async fn create_config() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
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
    // TODO: figure out if this is really needed
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

#[server]
pub async fn read_config() -> Result<AppConfig, ServerFnError> {
    create_config().await?;
    #[cfg(feature = "server")]
    let mut cfg = backend::config::Config::read().map_err(ServerFnError::new)?;
    #[cfg(not(feature = "server"))]
    let mut cfg = AppConfig { data_dir: None }; // placeholder

    // If a path (directory) is configured, validate it exists (or can be created) and open/create DB inside.
    if let Some(dir) = cfg.data_dir.clone() {
        // Always attempt to set path (handles both existing and non-existing directories). It will
        // create the directory tree if needed.
        #[cfg(feature = "server")]
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
    #[cfg(feature = "server")]
    {
        Ok(AppConfig::from(cfg))
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(cfg)
    }
}

#[server]
pub async fn write_config(config: AppConfig) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    backend::config::Config::from(config)
        .write()
        .map_err(ServerFnError::new)?;
    Ok(())
}

#[server]
pub async fn write_path(path: PathBuf) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    backend::database::set_db_path(path).map_err(ServerFnError::new)?;
    Ok(())
}

#[server]
pub async fn persist_path(path: PathBuf) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    backend::config::persist_config(backend::config::Config {
        data_dir: Some(path),
    })
    .map_err(ServerFnError::new)?;
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

#[cfg(feature = "server")]
use crate::services::config::{init_config, persist_config};
use crate::shared::types::Config;
use dioxus::prelude::*;
use std::path::PathBuf;

#[server]
pub async fn get_config() -> Result<Config, ServerFnError> {
    init_config().map_err(ServerFnError::new)
}

#[server]
pub async fn persist_path(path: PathBuf) -> Result<(), ServerFnError> {
    persist_config(Config {
        data_dir: Some(path),
    })
    .map_err(ServerFnError::new)
}

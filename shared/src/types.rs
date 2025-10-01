use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub data_dir: Option<PathBuf>,
}

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    // Directory chosen by user to store the application's SQLite file (library.db)
    pub data_dir: Option<PathBuf>,
}

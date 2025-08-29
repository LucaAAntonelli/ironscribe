use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data_dir: Option<PathBuf>,
}

pub fn init_config() -> anyhow::Result<Config> {
    // Create app's OS-specific config directories
    // Linux: /home/<user>/.config/ironscribe
    // Windows: C:\Users\<user>\Appdata\Roaming\ironscribe\config
    if let Some(proj_dirs) = ProjectDirs::from("", "", "ironscribe") {
        let config_directory_path = proj_dirs.config_dir();
        create_dir_all(config_directory_path)?;
        let config_file_path = config_directory_path.join("config.json");
        let config_file_path = config_file_path.as_path();

        // Create config file if it doesn't exist yet
        if std::path::Path::exists(std::path::Path::new(config_file_path)) {
            // If file existed before, it may contain values already
            let content =
                std::fs::read_to_string(config_file_path).context("failed to read config file!")?;
            let config: Config = serde_json::from_str(&content)
                .context("failed to parse file content to Config object!")?;
            return Ok(config); // config may still not contain path here!
        } else {
            // File doesn't exist yet -> need to create and will not contain values yet
            File::create(config_file_path)?;
            return Ok(Config { data_dir: None }); // file was just created -> guaranteed to be empty
        }
    }
    Err(anyhow!("Failed to generate config folder path!"))
}

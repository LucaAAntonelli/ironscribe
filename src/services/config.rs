#![cfg(feature = "server")]
use crate::shared::types::Config;
use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};

impl Config {
    pub fn file_path() -> anyhow::Result<PathBuf> {
        let proj_dirs =
            ProjectDirs::from("", "", "ironscribe").context("failed to determine config path")?;
        let config_path = proj_dirs.config_dir().join("config.json");
        Ok(config_path)
    }

    pub fn init_config() -> anyhow::Result<()> {
        let config_path = Self::file_path()?;
        create_dir_all(config_path.clone())?;
        if Path::exists(config_path.as_path()) {
            return Err(anyhow!("File already exists!"));
        } else {
            File::create(config_path)?;
        }
        Ok(())
    }

    pub fn read_config() -> anyhow::Result<Self> {
        let config_path = Self::file_path()?;
        let content =
            std::fs::read_to_string(config_path).context("Failed to read config file!")?;
        match serde_json::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => Err(anyhow!("Failed to deserialize Config: {}", e)),
        }
    }

    pub fn save_config(&self) -> anyhow::Result<()> {
        let config_path = Self::file_path()?;
        let content = serde_json::to_string(self).context("Failed to serialize Config!")?;
        std::fs::write(config_path, content)
            .context("Failed to write serialized config to file!")?;
        Ok(())
    }
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
            if let Ok(config) = serde_json::from_str(&content) {
                return Ok(config); // config contains a path here
            } else {
                return Ok(Config { data_dir: None });
            }
        } else {
            // File doesn't exist yet -> need to create and will not contain values yet
            File::create(config_file_path)?;
            return Ok(Config { data_dir: None }); // file was just created -> guaranteed to be empty
        }
    }
    Err(anyhow!("Failed to generate config folder path!"))
}

pub fn persist_config(config: Config) -> anyhow::Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "ironscribe") {
        let config_directory_path = proj_dirs.config_dir();
        create_dir_all(config_directory_path)?;
        let config_file_path = config_directory_path.join("config.json");
        let config_file_path = config_file_path.as_path();

        // Serialize config to JSON string
        let config_json_str = serde_json::to_string(&config)?;

        // Write JSON to config file
        std::fs::write(config_file_path, config_json_str)?;
        return Ok(());
    }
    Err(anyhow!("Failed to generate config folder path!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        #[cfg(target_os = "windows")]
        let expected_path =
            PathBuf::from("C:\\Users\\lucaa\\Appdata\\Roaming\\ironscribe\\config\\config.json");
        let path = Config::file_path().unwrap();
        assert_eq!(expected_path, path);
    }
}

use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    // Directory chosen by user to store the application's SQLite file (library.db)
    pub data_dir: Option<PathBuf>,
}
impl Config {
    pub fn file_path() -> anyhow::Result<PathBuf> {
        let proj_dirs =
            ProjectDirs::from("", "", "ironscribe").context("failed to determine config path")?;
        let config_path = proj_dirs.config_dir().join("config.json");
        Ok(config_path)
    }

    pub fn init() -> anyhow::Result<()> {
        let config_path = Self::file_path()?;
        if let Some(parent) = config_path.parent() {
            create_dir_all(parent)?;
        }
        if Path::exists(config_path.as_path()) {
            // If it's a file we are done; if it's a directory, that's an error from previous buggy runs
            if config_path.is_dir() {
                return Err(anyhow!(
                    "A directory exists where the config file should be: {config_path:?}"
                ));
            }
            return Ok(()); // already exists, nothing to do
        }
        // Write default empty config
        std::fs::write(
            &config_path,
            serde_json::to_string(&Config { data_dir: None })?,
        )?;
        Ok(())
    }

    pub fn read() -> anyhow::Result<Self> {
        let config_path = Self::file_path()?;
        if config_path.is_dir() {
            return Err(anyhow!(
                "Expected config file but found a directory at {config_path:?}. Please remove or rename it."
            ));
        }
        let content =
            std::fs::read_to_string(config_path.clone()).context("Failed to read config file!")?;
        if content.trim().is_empty() {
            // Treat empty file as default config
            return Ok(Config { data_dir: None });
        }
        match serde_json::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => {
                tracing::warn!(
                    "Config file malformed ({}). Rewriting default & continuing.",
                    e
                );
                let default = Config { data_dir: None };
                std::fs::write(&config_path, serde_json::to_string(&default)?)?;
                Ok(default)
            }
        }
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let config_path = Self::file_path()?;
        if let Some(parent) = config_path.parent() {
            create_dir_all(parent)?;
        }
        let content = serde_json::to_string(self).context("Failed to serialize Config!")?;
        std::fs::write(config_path, content)
            .context("Failed to write serialized config to file!")?;
        Ok(())
    }

    pub fn file_exists() -> anyhow::Result<bool> {
        let config_path = Self::file_path()?;
        Ok(Path::exists(config_path.as_path()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        #[cfg(target_os = "windows")]
        let expected_path =
            PathBuf::from("C:\\Users\\lucaa\\Appdata\\Roaming\\ironscribe\\config\\config.json");
        #[cfg(target_os = "linux")]
        let expected_path = PathBuf::from("/home/luca/.config/ironscribe/config/config.json");
        let path = Config::file_path().unwrap();
        assert_eq!(expected_path, path);
    }
}

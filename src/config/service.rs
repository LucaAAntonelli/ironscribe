#![cfg(feature = "server")]
use crate::config::Config;
use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};

impl Config {
    pub fn file_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", "ironscribe").context("failed to determine config path")?;
        let config_path = proj_dirs.config_dir().join("config.json");
        Ok(config_path)
    }

    pub fn init() -> anyhow::Result<()> {
        let config_path = Self::file_path()?;
        if let Some(parent) = config_path.parent() { create_dir_all(parent)?; }
        if Path::exists(config_path.as_path()) {
            if config_path.is_dir() { return Err(anyhow!("A directory exists where the config file should be: {:?}", config_path)); }
            return Ok(());
        }
        std::fs::write(&config_path, serde_json::to_string(&Config { data_dir: None })?)?;
        Ok(())
    }

    pub fn read() -> anyhow::Result<Self> {
        let config_path = Self::file_path()?;
        if config_path.is_dir() { return Err(anyhow!("Expected config file but found a directory at {:?}. Please remove or rename it.", config_path)); }
        let content = std::fs::read_to_string(config_path.clone()).context("Failed to read config file!")?;
        if content.trim().is_empty() { return Ok(Config { data_dir: None }); }
        match serde_json::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => {
                tracing::warn!("Config file malformed ({}). Rewriting default & continuing.", e);
                let default = Config { data_dir: None };
                std::fs::write(&config_path, serde_json::to_string(&default)?)?;
                Ok(default)
            }
        }
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let config_path = Self::file_path()?;
        if let Some(parent) = config_path.parent() { create_dir_all(parent)?; }
        let content = serde_json::to_string(self).context("Failed to serialize Config!")?;
        std::fs::write(config_path, content).context("Failed to write serialized config to file!")?;
        Ok(())
    }

    pub fn file_exists() -> anyhow::Result<bool> {
        let config_path = Self::file_path()?;
        Ok(Path::exists(config_path.as_path()))
    }
}

pub fn init_config() -> anyhow::Result<Config> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "ironscribe") {
        let config_directory_path = proj_dirs.config_dir();
        create_dir_all(config_directory_path)?;
        let config_file_path = config_directory_path.join("config.json");
        let config_file_path = config_file_path.as_path();
        if std::path::Path::exists(std::path::Path::new(config_file_path)) {
            let content = std::fs::read_to_string(config_file_path).context("failed to read config file!")?;
            if let Ok(config) = serde_json::from_str(&content) { return Ok(config); } else { return Ok(Config { data_dir: None }); }
        } else {
            File::create(config_file_path)?;
            return Ok(Config { data_dir: None });
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
        let config_json_str = serde_json::to_string(&config)?;
        std::fs::write(config_file_path, config_json_str)?;
        return Ok(());
    }
    Err(anyhow!("Failed to generate config folder path!"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_path_includes_config_json() {
        let path = Config::file_path().unwrap();
        assert!(path.ends_with("config.json"));
    }
}

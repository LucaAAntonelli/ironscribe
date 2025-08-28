use anyhow::anyhow;
use directories::ProjectDirs;
use std::fs::{create_dir_all, File};

pub fn init_config() -> anyhow::Result<bool> {
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
            return Ok(true);
        } else {
            // File doesn't exist yet -> need to create and will not contain values yet
            File::create(config_file_path)?;
            return Ok(false);
        }
    }
    Err(anyhow!("Failed to generate config folder path!"))
}

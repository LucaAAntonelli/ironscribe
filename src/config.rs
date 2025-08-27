use directories::ProjectDirs;

pub fn init_config() {
    // Create app's OS-specific config directories
    // Linux: /home/<user>/.config/ironscribe
    // Windows: C:\Users\<user>\Appdata\Roaming\ironscribe\config
    if let Some(proj_dirs) = ProjectDirs::from("", "", "ironscribe") {
        println!("{proj_dirs:?}");
    }
}

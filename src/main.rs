mod config;
mod database;
mod frontend;
// Temporary: legacy modules kept for backward compatibility during refactor
mod backend;
mod services;
mod shared;

use crate::frontend::components::App;
#[cfg(feature = "server")]
use crate::config::init_config;

fn main() {
    #[cfg(feature = "server")]
    {
        if let Err(e) = init_config() {
            eprintln!("fatal: {e}");
            std::process::exit(1);
        }
    }
    dioxus::launch(App);
}

pub mod model;
pub mod service;
#[cfg(feature = "server")]
pub mod api;

pub use model::*;
#[cfg(feature = "server")]
pub use service::*;
#[cfg(feature = "server")]
pub use api::{create_config, read_config, write_config, write_path, persist_path};

// When not building with the `server` feature, provide no-op async stubs so the
// frontend components can still compile & run (they will just show loading / empty states).
#[cfg(not(feature = "server"))]
pub async fn read_config() -> Result<Config, dioxus::prelude::ServerFnError> {
	Ok(Config { data_dir: None })
}

#[cfg(not(feature = "server"))]
pub async fn write_path(_p: std::path::PathBuf) -> Result<(), dioxus::prelude::ServerFnError> { Ok(()) }

#[cfg(not(feature = "server"))]
pub async fn write_config(_c: Config) -> Result<(), dioxus::prelude::ServerFnError> { Ok(()) }

#[cfg(not(feature = "server"))]
pub async fn create_config() -> Result<(), dioxus::prelude::ServerFnError> { Ok(()) }

#[cfg(not(feature = "server"))]
pub async fn persist_path(_p: std::path::PathBuf) -> Result<(), dioxus::prelude::ServerFnError> { Ok(()) }

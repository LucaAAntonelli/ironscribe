#![cfg(feature = "server")]
// Deprecated module. All config functionality moved to crate::config::{model, service, api}.
pub use crate::config::{init_config, persist_path as persist_config, Config};

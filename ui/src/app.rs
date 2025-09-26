use crate::{books::Books, path_picker::Modal};
use api::config::{init_config_server, read_config, write_path};
use dioxus::prelude::*;
use std::path::PathBuf;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("assets/main.css");

#[component]
pub fn App() -> Element {
    let mut committed = use_signal(String::new);

    // Kick off config initialization (idempotent) via a server function so that
    // the web crate no longer calls backend code directly in its main(). We ignore
    // the result here; any errors will surface again when read_config runs.
    let init_cfg = use_server_future(|| init_config_server())?;
    use_effect(move || {
        if let Some(Err(e)) = init_cfg() {
            tracing::warn!("Config init failed: {}", e);
        }
    });

    // Config future; key it on a state value so we can refetch after user input
    let config_reload_key = use_signal(|| 0u64);
    let books_reload_key = use_signal(|| 0u64);
    let cfg = use_server_future(move || {
        let _k = config_reload_key();
        read_config()
    })?;

    // Log config state changes
    use_effect({
        move || match cfg() {
            Some(Ok(c)) => tracing::debug!("Config loaded: data_dir={:?}", c.data_dir),
            Some(Err(e)) => tracing::warn!("Config error: {}", e),
            None => {}
        }
    });

    let contents = match cfg() {
        None => rsx! {
            div { "Loading configuration..." }
        },
        Some(Err(e)) => rsx! {
            div { "Config error: {e}" }
        },
        Some(Ok(c)) => {
            if c.data_dir.is_some() {
                rsx! {
                    Books { reload_key: books_reload_key() }
                }
            } else {
                rsx! {
                    div { style: "min-height:100vh; display:grid; place-items:center; font-family:sans-serif;",
                        "Configure a data directory to continue."
                    }
                }
            }
        }
    };

    let show_modal =
        matches!(cfg(), Some(Ok(c)) if c.data_dir.is_none()) || matches!(cfg(), Some(Err(_)));

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        {contents}
        if show_modal {
            Modal {
                title: "Enter Data Directory",
                on_close: move || {},
                on_commit: move |s: String| {
                    committed.set(s.clone());
                    spawn({
                        let mut config_reload_key = config_reload_key.to_owned();
                        let mut books_reload_key = books_reload_key.to_owned();
                        async move {
                            match write_path(PathBuf::from(&s)).await {
                                Ok(_) => {
                                    config_reload_key.set(config_reload_key() + 1);
                                    books_reload_key.set(books_reload_key() + 1);
                                }
                                Err(e) => eprintln!("Failed to persist config: {e}"),
                            }
                        }
                    });
                },
            }
        }
    }
}

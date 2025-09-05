use crate::backend::config::get_config;
use crate::backend::config::persist_path;
use crate::frontend::components::Books;
use crate::frontend::components::Modal;
use dioxus::prelude::*;
use std::path::PathBuf;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Books,
}

#[component]
pub fn App() -> Element {
    // Toggles whether dialog shows up or not
    let mut show = use_signal(|| false);
    // Used to commit text box input in one go
    let mut committed = use_signal(String::new);
    // use_effect is triggered every time `committed` changes => auto-uptade for log
    use_effect({
        move || {
            tracing::info!("[committed changed]: {}", committed());
        }
    });

    let cfg = use_server_future(get_config)?;

    use_effect({
        move || match cfg() {
            Some(Ok(config)) => show.set(config.data_dir.is_none()),

            Some(Err(e)) => tracing::info!("Found error: {}", e),
            None => tracing::info!("Loading..."),
        }
    });

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
        if show() {
            div {
                style: "min-height:100vh; display:grid; place-items:center; font-family:sans-serif;",
            }
            Modal{
                title: "Enter Path",
                on_close: move || show.set(false),
                on_commit: move |s: String| {
                    committed.set(s.clone());
                    spawn(async move {
                        match persist_path(PathBuf::from(s)).await {
                            Ok(_) => {},
                            Err(e) => eprintln!("Failed to persist config: {e}")
                        }
                    });
                }
            }
        }
    }
}

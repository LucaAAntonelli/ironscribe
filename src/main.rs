mod backend;
mod components;
mod config;
mod types;
use backend::get_config;
use components::Books;
use components::Modal;
#[cfg(feature = "server")]
use config::init_config;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Books,
}

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

#[component]
fn App() -> Element {
    // Toggles whether dialog shows up or not
    let mut show = use_signal(|| true);
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
                on_commit: move |s| committed.set(s),
            }
        }
    }
}

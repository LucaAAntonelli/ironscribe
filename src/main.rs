mod backend;
mod components;
mod config;
mod types;
use backend::get_config;
use components::Books;
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
    let cfg = use_server_future(get_config)?;
    match cfg() {
        Some(Ok(config)) => tracing::info!("Found config: {:?}", config),
        Some(Err(e)) => tracing::info!("Found error: {}", e),
        None => tracing::info!("Loading..."),
    }
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}

mod backend;
mod components;
mod config;
use components::Books;
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
        match init_config() {
            Ok(config) => {
                dbg!(config);
            }
            Err(e) => {
                eprintln!("fatal: {e}");
                std::process::exit(1);
            }
        }
    }
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}

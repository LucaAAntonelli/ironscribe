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
    if let Ok(result) = init_config() {
        if result {
            dioxus::launch(App);
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}

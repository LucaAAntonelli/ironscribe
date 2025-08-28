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
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Err(e) = init_config() {
            eprintln!("fatal: {e}");
            std::process::exit(1); // exit with non-zero exit code
        } else {
            println!("Successfully created config folder and file");
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

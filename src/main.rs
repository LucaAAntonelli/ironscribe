mod backend;
mod components;
use components::Books;
use components::Favorites;
use components::NavBar;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    #[route("/books")]
    Books,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON}

        Router::<Route> {}

    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });
    rsx! {
        div {
            id: "dogview",
            img { src: img_src.cloned().unwrap_or_default()}
        }
        div {
            id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip"},
            button {
                id: "save",
                onclick: move |_| async move {

                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = backend::save_dog(current).await;
                },
            "save!"}
        }
    }
}

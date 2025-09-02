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
    // Toggles whether dialog shows up or not
    let mut show = use_signal(|| true);
    // Used to commit text box input in one go
    let mut committed = use_signal(|| String::new());
    use_effect({
        move || {
            tracing::info!("[committed changed]: {}", committed());
        }
    });

    let cfg = use_server_future(get_config)?;
    match cfg() {
        Some(Ok(config)) => {
            tracing::info!("Found config: {:?}", config);
            show.set(config.data_dir.is_none());
        }
        Some(Err(e)) => tracing::info!("Found error: {}", e),
        None => tracing::info!("Loading..."),
    }
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
        div {
            style: "min-height:100vh; display:grid; place-items:center; font-family:sans-serif;",

            button {
                onclick: move |_| show.set(true),
            }
        }
        if show() {
            Modal{
                title: "Enter Path",
                on_close: move || show.set(false),
                on_commit: move |s| committed.set(s),
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ModalProps {
    title: String,
    on_close: EventHandler<()>,
    on_commit: EventHandler<String>,
}

#[component]
fn Modal(props: ModalProps) -> Element {
    let mut draft = use_signal(|| String::new());
    rsx! {
        // Backdrop
        div {
            tabindex: 0,
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.45); \
                    display:flex; align-items:center; justify-content:center;",

            // Dialog
            div {
                role: "dialog",
                "aria-modal": "true",
                style: "background:white; width:300px; border-radius:10px; \
                        box-shadow:0 18px 40px rgba(0,0,0,0.25); padding:16px;",

                h2 { style: "margin:0 0 10px 0; font-size:18px; font-weight:600; color:black;", "{props.title}" }

                input {
                    r#type: "text",
                    value: "{draft}",
                    style: "width:100%; padding:8px; border:1px solid #ccc; border-radius:6px;",
                    autofocus: true,
                    oninput: move |e| draft.set(e.value()),
                    onkeydown: {
                        let on_commit = props.on_commit;
                        let on_close = props.on_close;
                        move |e: KeyboardEvent| {
                            if e.key() == dioxus::events::Key::Enter {
                                on_commit.call(draft());
                                on_close.call(());
                                e.prevent_default();
                            }
                        }
                    }
                },

            }
        }
    }
}

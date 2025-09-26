use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    title: String,
    on_close: EventHandler<()>,
    on_commit: EventHandler<String>,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut draft = use_signal(String::new);
    rsx! {
        // Backdrop
    div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.45); \
            display:flex; align-items:center; justify-content:center; z-index:1000;",

            // Dialog
            div {
                role: "dialog",
                "aria-modal": "true",
        style: "background:white; width:320px; border-radius:10px; \
            box-shadow:0 18px 40px rgba(0,0,0,0.25); padding:16px; pointer-events:auto;",

                h2 { style: "margin:0 0 10px 0; font-size:18px; font-weight:600; color:black;", "{props.title}" }

                input {
                    r#type: "text",
                    placeholder: "C:/path/to/folder",
                    style: "width:100%; padding:8px; border:1px solid #ccc; border-radius:6px; margin-bottom:8px;",
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
                }
                button {
                    style: "width:100%; padding:8px; background:#2563eb; color:white; border:none; border-radius:6px; cursor:pointer;",
                    onclick: move |_| {
                        props.on_commit.call(draft());
                        props.on_close.call(());
                    },
                    "Save"
                }

            }
        }
    }
}

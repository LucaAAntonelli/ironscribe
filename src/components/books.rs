use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn Books() -> Element {
    let books = use_server_future(crate::backend::list_books)?;

    rsx! {
        div { id: "books",
            div { id: "books-container",
                table {
                    thead {
                        tr {
                            th { onclick: move |event| tracing::info!("Clicked Title"), "Title" }
                            th { onclick: move |event| tracing::info!("Clicked Author"), "Author"}
                            th { onclick: move |event| tracing::info!("Clicked Series & Volume"), "Series & Volume" }
                        }
                    }
                    tbody {
                        match books() {
                            Some(Ok(books)) => rsx! {
                                for book in books {
                                    tr {
                                        td { "{book.get_title()}" }
                                        td { "{book.get_authors().join(\", \")}" }
                                        td { "{book.get_series_and_volumes().iter().join(\", \")}" }
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                tr {td { colspan: "3", "Error: {e}"}}
                            },
                            None => rsx! {
                                tr {td { colspan: "3", "Loading..."}}
                            }

                        }

                    }

                }
            }
        }
    }
}

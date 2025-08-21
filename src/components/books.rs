use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn Books() -> Element {
    let books = use_resource(crate::backend::list_books).suspend()?;

    rsx! {
        div { id: "books",
            div { id: "books-container",
                table {
                    thead {
                        tr {
                            th { onclick: move |event| tracing::info!("Clicked Title Header"), "Title" }
                            th { onclick: move |event| tracing::info!("Clicked Author Header"), "Author"}
                            th { onclick: move |event| tracing::info!("Clicked Series & Volume Header"), "Series & Volume" }
                        }
                    }
                    tbody {
                        for book in books().unwrap() {
                            tr {
                                td { "{book.get_title()}" }
                                td { "{book.get_authors().join(\", \")}" }
                                td { "{book.get_series_and_volumes().iter().join(\", \")}" }
                            }
                        }
                    }

                }
            }
        }
    }
}

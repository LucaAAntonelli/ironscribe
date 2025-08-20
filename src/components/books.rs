use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn Books() -> Element {
    let books = use_resource(crate::backend::list_books).suspend()?;

    rsx! {
        div { id: "books",
            div { id: "books-container",
                table {
                    tr {
                        th { "Title" }
                        th { "Author" }
                        th { "Series & Volume" }
                    }
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

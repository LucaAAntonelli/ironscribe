use dioxus::prelude::*;

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
                    for (title, author, series_and_volume) in books().unwrap() {
                        tr {
                            td { "{title}" }
                            td { "{author}" }
                            td { "{series_and_volume}" }
                        }
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Books() -> Element {
    let mut books = use_resource(crate::backend::list_books).suspend()?;

    rsx! {
        div { id: "books",
            div { id: "books-container",
                    style: "overflow-y:auto",
                table {
                    tr {
                        th {
                            "Book Title"
                        }
                    }
                    for title in books().unwrap() {
                        tr {
                            td {
                                "{title}"
                            }
                        }
                    }
                }
            }
        }
    }
}

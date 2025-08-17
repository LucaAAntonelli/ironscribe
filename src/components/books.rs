use dioxus::prelude::*;

#[component]
pub fn Books() -> Element {
    let books = use_resource(crate::backend::list_books).suspend()?;

    rsx! {
        div { id: "books",
            div { id: "books-container",
                    style: "overflow-y:auto",
                table {
                    tr {
                        th {
                            "Title"
                        }
                        th {
                            "Author"
                        }
                    }
                    for (title, author) in books().unwrap() {
                        tr {
                            td {
                                "{title}"
                            }
                            td {
                                "{author}"
                            }
                        }
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;
use itertools::Itertools;

use crate::backend::{sort_books, SortKey};

struct SortState {
    key: SortKey,
    ascending: bool,
}

#[component]
pub fn Books() -> Element {
    let books = use_server_future(crate::backend::list_books)?;

    let mut sort_state = use_signal(|| SortState {
        key: SortKey::DateAdded,
        ascending: false,
    }); // Default sorting

    rsx! {
        div { id: "books",
            div { id: "books-container",
                match books() {
                    Some(Err(e)) => rsx! {
                        div { "Error: {e}" }
                    },
                    None => rsx! {
                        div { "Loading..." }
                    },
                    Some(Ok(books)) =>  {
                        let mut sorted = books.clone();
                        sorted = sort_books(&mut sorted, sort_state.read().key.clone(), sort_state.read().ascending);

                        rsx! {
                            table {
                                thead {
                                    tr {
                                        th { onclick: move |_| {
                                            tracing::info!("Title clicked!");
                                            let mut write = sort_state.write();
                                            write.key = SortKey::Title;
                                            write.ascending = !write.ascending;
                                        }, "Title" }
                                        th { onclick: move |_| {
                                            tracing::info!("Author clicked!");
                                            let mut write = sort_state.write();
                                            write.key = SortKey::Author;
                                            write.ascending = !write.ascending;
                                        }, "Author" }
                                        th { onclick: move |_| {
                                            tracing::info!("Series & Volume clicked!");
                                            let mut write = sort_state.write();
                                            write.key = SortKey::SeriesAndVolume;
                                            write.ascending = !write.ascending;
                                        }, "Series & Volume" }
                                    }
                                }
                                tbody {
                                    for book in sorted {
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
        }
    }
}

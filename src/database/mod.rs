pub mod models;
#[cfg(feature = "server")]
pub mod connection;
#[cfg(feature = "server")]
pub mod api;

pub use models::*;
#[cfg(feature = "server")]
pub use connection::*;
#[cfg(feature = "server")]
pub use api::list_books;

#[cfg(not(feature = "server"))]
pub async fn list_books() -> Result<BookRecords, dioxus::prelude::ServerFnError> {
	Ok(BookRecords { records: vec![] })
}

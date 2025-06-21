use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router};
use crate::shared::BookStore;
use crate::grpc::booksync::Book;
use std::sync::Arc;

#[axum::debug_handler]
pub fn routes(store: Arc<BookStore>) -> Router {
    Router::new()
        .route("/upload", post(upload_book))
        .route("/book/:id", get(get_book))
        .with_state(store.clone())
}

#[axum::debug_handler]
async fn upload_book(
    State(store): State<Arc<BookStore>>,
    Json(book): Json<Book>,
) -> Json<&'static str> {
    store.insert(book).await;
    Json("uploaded")
}

#[axum::debug_handler]
async fn get_book(
    State(store): State<Arc<BookStore>>,
    Path(id): Path<String>
    // TODO: Option<Json<Book>> doesn't implement IntoResponse trait -> Implement or change types
) -> Option<Json<Book>> {
    store.get(&id).await.map(Json)
}

#[axum::debug_handler]
pub async fn handler(
    State(_state): State<Arc<BookStore>>,
) {
    println!("Getter!");
}
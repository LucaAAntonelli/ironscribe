use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use crate::shared::BookStore;
use crate::grpc::booksync::Book;
use std::sync::Arc;

pub fn routes(store: Arc<BookStore>) -> Router {
    Router::new()
        .route("/upload", post(upload_book))
        .route("/book/:id", get(get_book))
        .with_state(store)
}

async fn upload_book(
    State(store): State<Arc<BookStore>>,
    Json(book): Json<Book>,
) -> Json<&'static str> {
    store.insert(book).await;
    Json("uploaded")
}

async fn get_book(
    Path(id): Path<String>,
    State(store): State<Arc<BookStore>>,
) -> Option<Json<Book>> {
    store.get(&id).await.map(Json)
}

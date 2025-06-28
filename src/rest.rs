use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use crate::shared::BookStore;
use crate::grpc_server::booksync::Book;
use std::sync::Arc;

pub fn routes(store: Arc<BookStore>) -> Router {
    Router::new()
        .route("/upload", post(upload_book))
        .route("/book/{id}", get(get_book))
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
) -> impl IntoResponse {
    match store.get(&id).await {
        Some(book) => Json(book).into_response(),
        None => StatusCode::NOT_FOUND.into_response()
    }
}

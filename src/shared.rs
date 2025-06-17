use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::grpc::booksync::Book;

#[derive(Clone, Default, Debug)]
pub struct BookStore {
    inner: Arc<RwLock<HashMap<String, Book>>>,
}

impl BookStore {
    pub async fn insert(&self, book: Book) {
        self.inner.write().await.insert(book.id.clone(), book);
    }

    pub async fn get(&self, id: &str) -> Option<Book> {
        self.inner.read().await.get(id).cloned()
    }
}

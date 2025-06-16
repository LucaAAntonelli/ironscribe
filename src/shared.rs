use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub format: String,
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

#[derive(Clone, Default)]
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

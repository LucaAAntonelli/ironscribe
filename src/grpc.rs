use tonic::{Request, Response, Status};
use crate::shared::{BookStore, Book};
use booksync::book_sync_server::{BookSync, BookSyncServer};
use booksync::{UploadBookRequest, UploadBookResponse, GetBookRequest, GetBookResponse};

pub mod booksync {
    tonic::include_proto!("booksync");
}

#[derive(Debug)]
pub struct MyBookSync {
    pub store: BookStore,
}

#[tonic::async_trait]
impl BookSync for MyBookSync {
    async fn upload_book(
        &self,
        request: Request<UploadBookRequest>,
    ) -> Result<Response<UploadBookResponse>, Status> {
        let book = request.into_inner().book.unwrap();
        self.store.insert(book.clone()).await;
        Ok(Response::new(UploadBookResponse {
            status: "uploaded".into(),
        }))
    }

    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        let id = request.into_inner().id;
        match self.store.get(&id).await {
            Some(book) => Ok(Response::new(GetBookResponse { book: Some(book) })),
            None => Err(Status::not_found("Book not found")),
        }
    }
}

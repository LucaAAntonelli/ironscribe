use anyhow::anyhow;
use shared::proto::{
    AddBookRequest, AddBookResponse, DeleteBookRequest, DeleteBookResponse, UpdateBookRequest,
    UpdateBookResponse, add_book_request, book_sync_server::BookSync,
};
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tonic::{Code, Request, Response, Status, Streaming};
use tracing::{error, instrument};

#[derive(Debug)]
pub struct BookServer {
    directory: Arc<PathBuf>,
}

impl BookServer {
    #[instrument]
    pub fn new(directory: PathBuf) -> Self {
        println!("Creating BookServer instance!");
        Self {
            directory: Arc::new(directory),
        }
    }

    pub fn get_directory(&self) -> String {
        self.directory.to_str().to_owned().unwrap().to_string()
    }
}
// TODO: Add traces for more information about program execution
#[tonic::async_trait]
impl BookSync for BookServer {
    #[instrument]
    async fn add_book(
        &self,
        request: Request<Streaming<AddBookRequest>>,
    ) -> tonic::Result<Response<AddBookResponse>, Status> {
        let mut request_stream = request.into_inner();
        let directory = Arc::clone(&self.directory);
        println!("Received request for directory: {:?}", directory);

        let task_handle = tokio::spawn(async move {
            let file_name = if let Some(file_upload) = request_stream.next().await {
                match file_upload?.r#type.unwrap() {
                    add_book_request::Type::Name(name) => name,
                    wrong_type => Err(anyhow!("Wrong message type: {:?}", wrong_type))?,
                }
            } else {
                Err(anyhow!("Wrong message type"))?
            };

            let mut file_path = PathBuf::new();
            file_path.push(directory.as_ref());
            file_path.push(&file_name);

            let mut file_handle = fs::File::create(file_path).await?;

            while let Some(file_upload) = request_stream.next().await {
                match file_upload?.r#type {
                    Some(add_book_request::Type::Chunk(chunk)) => {
                        file_handle.write_all(&chunk).await?;
                    }
                    wrong_type => Err(anyhow!("Wrong message type: {:?}", wrong_type))?,
                }
            }

            file_handle.sync_all().await?;

            Ok::<(), anyhow::Error>(())
        });

        if let Err(err) = task_handle.await.unwrap() {
            error!(%err);
            Err(Status::internal("Failed to upload file"))
        } else {
            println!("Successfully added book!");
            Ok(Response::new(AddBookResponse::default()))
        }
    }

    async fn delete_book(
        &self,
        request: Request<DeleteBookRequest>,
    ) -> tonic::Result<Response<DeleteBookResponse>, Status> {
        // Client sends request with path of a deleted file -> delete on server too
        let request = request.into_inner();
        let path = PathBuf::from(request.path);
        if !path.exists() {
            // Path doesn't exist
            return Err(Status::new(
                Code::NotFound,
                format!("File {path:?} doesn't exist, cannot delete!"),
            ));
        }
        // Path exists -> try to delete and return result
        if path.is_file() {
            match std::fs::remove_file(path.clone()) {
                Ok(_) => {
                    return Ok(Response::new(DeleteBookResponse::default()));
                }
                Err(e) => {
                    return Err(Status::new(
                        tonic::Code::Unknown,
                        format!("Error deleting file {path:?}: {e}"),
                    ));
                }
            }
        }
        // Path is directory -> try to delete and return result
        match std::fs::remove_dir_all(path.clone()) {
            Ok(_) => {
                return Ok(Response::new(DeleteBookResponse::default()));
            }
            Err(e) => {
                return Err(Status::new(
                    Code::Unknown,
                    format!("Error deleting directory {path:?}: {e}"),
                ));
            }
        }
    }

    async fn update_book(
        &self,
        request: Request<Streaming<UpdateBookRequest>>,
    ) -> tonic::Result<Response<UpdateBookResponse>, Status> {
        Ok(Response::new(UpdateBookResponse::default()))
    }
}

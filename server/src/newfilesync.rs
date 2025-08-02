use anyhow::anyhow;
use shared::proto::{
    AddBookRequest, AddBookResponse, DeleteBookRequest, DeleteBookResponse, ListBooksRequest,
    ListBooksResponse, UpdateBookRequest, UpdateBookResponse, add_book_request,
    book_sync_server::BookSync,
};
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::{Code, Request, Response, Status, Streaming};
use tracing::{Instrument, error, instrument};

#[derive(Debug)]
pub struct BookServer {
    directory: Arc<PathBuf>,
}

impl BookServer {
    const CHANNEL_SIZE: usize = 10;
    const CHUNK_BYTE_SIZE: u64 = 1024 * 1024;
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
#[tonic::async_trait]
impl BookSync for BookServer {
    type ListBooksStream = ReceiverStream<Result<ListBooksResponse, Status>>;

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

    #[instrument]
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

    #[instrument]
    async fn update_book(
        &self,
        request: Request<Streaming<UpdateBookRequest>>,
    ) -> tonic::Result<Response<UpdateBookResponse>, Status> {
        Ok(Response::new(UpdateBookResponse::default()))
    }

    #[instrument(skip(self))]
    async fn list_books(
        &self,
        _request: Request<ListBooksRequest>,
    ) -> Result<Response<Self::ListBooksStream>, Status> {
        let (tx, rx) = mpsc::channel(Self::CHANNEL_SIZE);
        let directory = Arc::clone(&self.directory);
        let tx_error = tx.clone();

        tokio::spawn(
            async move {
                let result = async move {
                    let mut dir_stream = fs::read_dir(directory.as_ref()).await?;

                    while let Some(dir_entry) = dir_stream.next_entry().await? {
                        let file_metadata = dir_entry.metadata().await?;
                        if !file_metadata.is_file() {
                            continue;
                        }
                        let file_name = dir_entry.file_name().into_string().map_err(|e| {
                            anyhow!("OsString convertion failed: {:?}", e.to_string_lossy())
                        })?;
                        let file_size = file_metadata.len();

                        if let Err(err) = tx
                            .send(Ok(ListBooksResponse {
                                name: file_name,
                                size: file_size,
                            }))
                            .await
                        {
                            error!(%err);
                            break;
                        }
                    }

                    Ok::<(), anyhow::Error>(())
                }
                .await;

                if let Err(err) = result {
                    error!(%err);
                    let send_result = tx_error
                        .send(Err(Status::internal("Failed to list files")))
                        .await;

                    if let Err(err) = send_result {
                        error!(%err);
                    }
                }
            }
            .in_current_span(),
        );

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

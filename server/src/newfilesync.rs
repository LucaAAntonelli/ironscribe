use anyhow::{Result, anyhow};
use shared::proto::{
    AddBookRequest, AddBookResponse, add_book_request, book_sync_server::BookSync,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tonic::transport::{Certificate, Identity, ServerTlsConfig};
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, instrument};

fn create_tls_config(
    cert_path: &Path,
    key_path: &Path,
    ca_cert_path: &Path,
) -> Result<ServerTlsConfig> {
    let cert = std::fs::read_to_string(cert_path)?;
    let key = std::fs::read_to_string(key_path)?;
    let ca_cert = std::fs::read_to_string(ca_cert_path)?;

    let identity = Identity::from_pem(cert, key);
    let client_ca_cert = Certificate::from_pem(ca_cert);
    let tls_config = ServerTlsConfig::new()
        .identity(identity)
        .client_ca_root(client_ca_cert);

    Ok(tls_config)
}

#[derive(Debug)]
pub struct BookServer {
    directory: Arc<PathBuf>,
}

impl Default for BookServer {
    fn default() -> Self {
        Self {
            directory: Arc::new(PathBuf::from(
                "C:\\Users\\lucaa\\Projects\\ironscribe\\TESTING",
            )),
        }
    }
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
}

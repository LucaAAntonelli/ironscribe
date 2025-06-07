use tonic::{transport::Server, Request, Response, Status};
use file_transfer::file_transfer_server::{FileTransfer, FileTransferServer};
use file_transfer::{FileUploadRequest, UploadStatus};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub mod file_transfer {
    tonic::include_proto!("filetransfer"); // matches proto package name
}

#[derive(Default)]
pub struct MyFileTransfer {}

#[tonic::async_trait]
impl FileTransfer for MyFileTransfer {
    async fn upload(
        &self,
        request: Request<tonic::Streaming<FileUploadRequest>>,
    ) -> Result<Response<UploadStatus>, Status> {
        let mut stream = request.into_inner();
        let mut total_size: i64 = 0;
        let upload_id = Uuid::new_v4().to_string();
        let mut filename = String::new();
        let mut file = None;

        while let Some(chunk) = stream
            .message()
            .await
            .map_err(|e| Status::internal(format!("Stream error: {}", e)))?
        {
            if file.is_none() {
                filename = chunk.file_name.clone();
                let path = format!("./uploads/{}-{}", upload_id, &filename);
                let f = File::create(path)
                    .await
                    .map_err(|e| Status::internal(format!("File create error: {}", e)))?;
                file = Some(f);
            }

            let data = chunk.chunk;
            total_size += data.len() as i64;
            if let Some(f) = file.as_mut() {
                f.write_all(&data)
                    .await
                    .map_err(|e| Status::internal(format!("Write error: {}", e)))?;
            }
        }

        Ok(Response::new(UploadStatus {
            file_name: filename,
            size: total_size,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc = FileTransferServer::new(MyFileTransfer::default());
    println!("Server listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}

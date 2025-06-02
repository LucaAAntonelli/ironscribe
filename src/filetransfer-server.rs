use tonic::{transport::Server, Request, Response, Status, Streaming};

use filetransfer::file_service_server::{FileService, FileServiceServer};
use filetransfer::{FileUploadRequest, FileUploadResponse};

pub mod filetransfer {
    tonic::include_proto!("filetransfer");
}

#[derive(Debug, Default)]
pub struct MyFileServer {}

#[tonic::async_trait]
impl FileService for MyFileServer {
    async fn upload(
        &self, 
        request: Request<Streaming<FileUploadRequest>>,
    ) -> Result<Response<FileUploadResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = FileUploadResponse {
            filename: "bleh".to_string(),
            size: 999
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let fileserver = MyFileServer::default();

    Server::builder().add_service(FileServiceServer::new(fileserver)).serve(addr).await?;

    Ok(())
}
use tonic::{transport::Server, Request, Response, Status};

use file_sync::dir_sync_server::{DirSync, DirSyncServer};

use crate::file_sync::{HelloRequest, HelloResponse};

pub mod file_sync {
    tonic::include_proto!("service");
}
#[derive(Debug, Default)]
pub struct MyDirSync {}

#[tonic::async_trait]
impl DirSync for MyDirSync {
    async fn sayff_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let dirsync = MyDirSync::default();

    Server::builder().add_service(DirSyncServer::new(dirsync)).serve(addr).await?;
    
    Ok(())
}
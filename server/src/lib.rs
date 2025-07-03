use tonic::{Request, Response, Status, Streaming};
use shared::shared::{dir_sync_server::DirSync, Block, ChecksumRequest, ChecksumResponse, DiffRequest, DiffResponse, HelloRequest, HelloResponse, SyncRequest, SyncResponse, UploadResponse};

#[derive(Debug, Default)]
pub struct MyDirSync {}

#[tonic::async_trait]
impl DirSync for MyDirSync {
    
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }

    async fn sync_structure(
        &self,
        request: Request<SyncRequest>,
    ) -> Result<Response<SyncResponse>, Status> {
        todo!("IMPLEMENT sync_structure()!");
    }

    async fn diff_structure(
        &self,
        request: Request<DiffRequest>,
    ) -> Result<Response<DiffResponse>, Status> {
        todo!("IMPLEMENT diff_structure()!");
    }

    async fn get_checksum(
        &self,
        request: Request<ChecksumRequest>,
    ) -> Result<Response<ChecksumResponse>, Status> {
        todo!("IMPLEMENT get_checksum()!");
    }

    async fn upload_blocks(
        &self,
        request: Request<Streaming<Block>>,
    ) -> Result<Response<UploadResponse>, Status> {
        todo!("IMPLEMENT upload_blocks()!");
    }
}



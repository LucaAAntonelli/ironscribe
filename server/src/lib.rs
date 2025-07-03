use tonic::{Request, Response, Status};

use shared::shared::{dir_sync_server::DirSync, HelloRequest, HelloResponse};

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
}



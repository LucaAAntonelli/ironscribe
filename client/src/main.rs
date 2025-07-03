use shared::shared::{HelloRequest, dir_sync_client::DirSyncClient};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DirSyncClient::connect("http://[::1]:50051").await?;

    let request = Request::new(HelloRequest {
        name: "Humbert".into()
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

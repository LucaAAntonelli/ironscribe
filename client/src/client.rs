use file_sync::dir_sync_client::DirSyncClient;
use file_sync::HelloRequest;

pub mod file_sync {
    tonic::include_proto!("service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DirSyncClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Humbert".into()
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
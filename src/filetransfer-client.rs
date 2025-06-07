use filetransfer::file_service_client::FileServiceClient;
use filetransfer::FileUploadRequest;

use tonic::Streaming;

pub mod filetransfer {
    tonic::include_proto!("filetransfer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Streaming::new(FileUploadRequest {
        filename: "myFile".to_string(),
        chunk: vec! [3 as u8]
    }));

    let response = client.upload(request).await?;

    println!("RESPONSE:{:?}", response);

    Ok(())

}
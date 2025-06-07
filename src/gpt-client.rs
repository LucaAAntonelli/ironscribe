use file_transfer::file_transfer_client::FileTransferClient;
use file_transfer::FileUploadRequest;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tonic::Request;

pub mod file_transfer {
    tonic::include_proto!("filetransfer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileTransferClient::connect("http://[::1]:50051").await?;
    let file_path = "my_big_file.bin";
    let mut file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; 1024 * 1024]; // 1 MB chunk
    let mut stream = client.upload(Request::new(tokio_stream::iter(
        async_stream::stream! {
            loop {
                let n = reader.read(&mut buffer).await?;
                if n == 0 { break; }
                let chunk = buffer[..n].to_vec();
                yield FileUploadRequest {
                    filename: file_path.to_string(),
                    chunk,
                };
            }
            Ok::<_, std::io::Error>(())
        },
    )))
    .await?
    .into_inner();

    println!("Upload complete: {} bytes", stream.size);
    Ok(())
}

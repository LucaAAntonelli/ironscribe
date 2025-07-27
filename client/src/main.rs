use client::newfilesync::BookClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BookClient::new("[::1]", 50051, None, None, None).await?;
    Ok(())
}

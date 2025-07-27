use client::newfilesync::BookClient;
use eframe::egui;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BookClient::new("[::1]", 50051, None, None, None).await?;
    Ok(())
}

struct MyApp {
    name: String,
    client: BookClient<Channel>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Hello World".to_string(),
            client: BookClient::new("[::1]", 50051, None, None, None).await?;
        }
    }
}

use server::MyDirSync;
use shared::shared::dir_sync_server::DirSyncServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let dirsync = MyDirSync::default();

    Server::builder().add_service(DirSyncServer::new(dirsync)).serve(addr).await?;
    
    Ok(())
}

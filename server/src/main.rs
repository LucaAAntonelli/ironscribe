use server::newfilesync::BookServer;
use shared::proto::book_sync_server::BookSyncServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let booksync = BookServer::default();

    Server::builder()
        .add_service(BookSyncServer::new(booksync))
        .serve(addr)
        .await?;

    Ok(())
}

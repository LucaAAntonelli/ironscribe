use anyhow::anyhow;
use server::newfilesync::BookServer;
use shared::proto::book_sync_server::BookSyncServer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tonic::transport::Server;
use tonic::transport::server::TcpIncoming;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50051);
    let listener = TcpListener::bind(socket_addr).await?;
    let local_addr = listener.local_addr()?;
    let listener = TcpIncoming::from_listener(listener, true, None).map_err(|e| anyhow!(e))?;
    debug!("Created server at directory {}", booksync.get_directory());

    Server::builder()
        .add_service(BookSyncServer::new(booksync))
        .serve(socket_addr)
        .serve_with_incoming(listener)
        .await?;

    Ok(())
}

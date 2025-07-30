use anyhow::anyhow;
use server::newfilesync::BookServer;
use shared::proto::book_sync_server::BookSyncServer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{path::PathBuf, str::FromStr};
use tokio::net::TcpListener;
use tonic::transport::Server;
use tonic::transport::server::TcpIncoming;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50051);
    let listener = TcpListener::bind(socket_addr).await?;
    let local_addr = listener.local_addr()?;
    let listener = TcpIncoming::from(listener);

    let file_service_impl = BookServer::new(
        PathBuf::from_str("C:\\Users\\Luca.Antonelli\\Documents\\Scripts\\ironscribe\\TESTING")
            .unwrap(),
    );
    let file_service_server = BookSyncServer::new(file_service_impl);

    Server::builder()
        .add_service(file_service_server)
        .serve_with_incoming(listener)
        .await?;

    Ok(())
}

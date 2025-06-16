mod grpc;
mod rest;
mod shared;

use grpc::booksync::book_sync_server::BookSyncServer;
use grpc::MyBookSync;
use rest::routes;
use shared::BookStore;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::try_join;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Arc::new(BookStore::default());

    let grpc_service = MyBookSync {
        store: store.clone(),
    };

    let grpc_addr = "127.0.0.1:50051".parse::<SocketAddr>()?;
    let rest_addr = "127.0.0.1:8080".parse::<SocketAddr>()?;

    let grpc_server = Server::builder()
        .add_service(BookSyncServer::new(grpc_service))
        .serve(grpc_addr);

    let rest_server = axum::Server::bind(&rest_addr)
        .serve(routes(store.clone()).into_make_service());

    println!("gRPC server on {grpc_addr}, REST on {rest_addr}");
    try_join!(grpc_server, rest_server)?;

    Ok(())
}

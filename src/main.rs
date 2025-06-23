mod grpc;
mod rest;
mod shared;

use grpc::booksync::book_sync_server::BookSyncServer;
use grpc::MyBookSync;
use rest::routes;
use shared::BookStore;
use std::net::SocketAddr;
use std::sync::Arc;
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

    let rest_app = routes(Arc::clone(&store));
    // let rest_app = axum::Router::new().route("/", axum::routing::get(handler)).with_state(store.clone());
    let rest_listener = tokio::net::TcpListener::bind(rest_addr).await.unwrap();

    println!("gRPC server on {grpc_addr}, REST on {rest_addr}");
    axum::serve(rest_listener, rest_app).await.unwrap();
    grpc_server.await.unwrap();

    Ok(())
}

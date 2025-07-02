use tonic::{transport::Server, Request, Response, Status};

use file_sync::dir_sync_server::{DirSync, DirSyncServer};

pub mod file_sync {
    tonic::include_proto!("service");
}
fn main() {
    println!("Hello world from server!");
}
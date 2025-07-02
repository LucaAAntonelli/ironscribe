use file_sync::dir_sync_client::DirSyncClient;

pub mod file_sync {
    tonic::include_proto!("service");
}

fn main() {
    println!("Hello world from client!");
}
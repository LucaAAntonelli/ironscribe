pub mod proto {
    tonic::include_proto!("service");
    tonic::include_proto!("library_sync");
}
pub mod filesystem;

pub mod errors;

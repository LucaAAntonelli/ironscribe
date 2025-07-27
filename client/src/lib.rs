pub mod newfilesync;
use chrono::Utc;
use shared::proto::dir_sync_client::DirSyncClient;
use std::{error::Error, path::PathBuf};
use tonic::transport::Channel;

struct Client {
    client: DirSyncClient<Channel>,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: DirSyncClient::connect("http://[::1]:50051").await?,
        })
    }
}

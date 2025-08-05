pub mod app;
pub mod db;
use anyhow::Result;
use db::DatabaseClient;
use shared::proto::{
    AddBookRequest, ListBooksRequest, add_book_request, book_sync_client::BookSyncClient,
};
use std::net::IpAddr;
use std::path::PathBuf;
use tokio::{fs, io::AsyncReadExt, sync::mpsc};
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::transport::{Certificate, ClientTlsConfig, Identity, channel::Channel};
use tracing::{Instrument, debug, error, instrument};

fn create_uri(host: &str, port: u16, tls: bool) -> String {
    let scheme = if tls { "https" } else { "http" };

    if let Ok(ip_addr) = host.parse::<IpAddr>() {
        return match ip_addr {
            IpAddr::V4(ipv4) => format!("{scheme}://{ipv4}:{port}"),
            IpAddr::V6(ipv6) => format!("{scheme}://[{ipv6}]:{port}"),
        };
    }

    format!("{scheme}://{host}:{port}")
}

fn create_tls_config(
    ca_cert_pem: &str,
    domain_name: &str,
    cert: &str,
    key: &str,
) -> Result<ClientTlsConfig> {
    let ca = Certificate::from_pem(ca_cert_pem);
    let identity = Identity::from_pem(cert, key);

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name(domain_name)
        .identity(identity);

    Ok(tls_config)
}

#[derive(Clone)]
pub struct BookClient<T> {
    sync_client: BookSyncClient<T>,
    db_client: DatabaseClient,
}

impl<T> BookClient<T> {
    const CHANNEL_SIZE: usize = 10;
    const CHUNK_SIZE_BYTES: u64 = 1024 * 1024;
}

impl BookClient<Channel> {
    #[instrument]
    pub async fn new(
        address: &str,
        port: u16,
        ca_cert_pem: Option<&str>,
        cert: Option<&str>,
        key: Option<&str>,
    ) -> Result<Self> {
        let enable_tls = ca_cert_pem.is_some() && cert.is_some() && key.is_some();
        let dst = create_uri(address, port, enable_tls);

        debug!("Connecting to {}", dst);

        let mut endpoint = Channel::from_shared(dst)?;

        if enable_tls {
            let tls_config =
                create_tls_config(ca_cert_pem.unwrap(), address, cert.unwrap(), key.unwrap())?;
            endpoint = endpoint.tls_config(tls_config)?;
        }

        let channel = endpoint.connect().await?;

        let client = BookSyncClient::new(channel);

        debug!("Connected");
        Ok(Self {
            sync_client: client,
            db_client: DatabaseClient {},
        })
    }

    // TODO: Change return type so result of function call can be used in GUI
    // TODO: Add more info/debug traces
    #[instrument(skip(self))]
    pub async fn add_book(&mut self, file: String, directory: PathBuf) -> Result<()> {
        let (tx, rx) = mpsc::channel(Self::CHANNEL_SIZE);

        let receiver_stream = ReceiverStream::new(rx);

        let mut file_path = PathBuf::new();
        file_path.push(&directory);
        file_path.push(&file);

        let task_handle = tokio::spawn(
            async move {
                if let Err(err) = tx
                    .send(AddBookRequest {
                        r#type: Some(add_book_request::Type::Name(file)),
                    })
                    .await
                {
                    error!(%err);
                    Err(err)?;
                }

                let file = fs::File::open(file_path).await?;
                let mut handle = file.take(Self::CHUNK_SIZE_BYTES);

                loop {
                    let mut chunk = Vec::with_capacity(Self::CHUNK_SIZE_BYTES as usize);

                    let n = handle.read_to_end(&mut chunk).await?;

                    if 0 == n {
                        break;
                    } else {
                        handle.set_limit(Self::CHUNK_SIZE_BYTES);
                    }

                    let request = AddBookRequest {
                        r#type: Some(add_book_request::Type::Chunk(chunk)),
                    };

                    if let Err(err) = tx.send(request).await {
                        error!(%err);
                        Err(err)?;
                    }

                    if n < Self::CHUNK_SIZE_BYTES as usize {
                        break;
                    }
                }

                Ok::<(), anyhow::Error>(())
            }
            .in_current_span(),
        );

        self.sync_client.add_book(receiver_stream).await?;

        if let Err(err) = task_handle.await? {
            error!(%err);
            Err(err)?;
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_books(&mut self) -> Result<()> {
        let mut books = Vec::new();

        let response = self.sync_client.list_books(ListBooksRequest {}).await?;

        let mut books_stream = response.into_inner();

        while let Some(book) = books_stream.next().await {
            books.push(book?);
        }

        println!("{:?}", books);

        Ok(())
    }
}

use axum;
use dioxus::prelude::*;
use tokio;
use ui::app::App;
fn main() {
    if let Err(e) = init_config() {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            launch_server(App).await;
        });
}

async fn launch_server(component: fn() -> Element) {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), component)
        .into_make_service();
    axum::serve(listener, router).await.unwrap();
}

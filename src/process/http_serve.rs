use anyhow::Result;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::path::Path;
use tracing::info;

pub async fn process_http_serve(path: &Path, port: u16) -> Result<()> {
    let router = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port{}", path, addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn index_handler() -> &'static str {
    "Hello, World!"
}

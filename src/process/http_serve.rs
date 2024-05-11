use anyhow::Result;
use axum::extract::State;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port{}", &path, addr);
    let state = HttpServeState { path };
    let router = Router::new()
        .route("/", get(index_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn index_handler(State(state): State<Arc<HttpServeState>>) -> String {
    format!("Serving {:?}", state.path)
}

use anyhow::Result;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Html};
use axum::{routing::get, Router};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port{}", &dir, addr);
    let state: HttpServeState = HttpServeState { path: dir.clone() };
    let service = ServeDir::new(dir)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd()
        .not_found_service(ServeFile::new("assets/not_found.html"));

    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    format!("Serving {:?}", state.path);
    let p = std::path::Path::new(&state.path).join(path);

    info!("Reading file {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, Html("File not found".to_string())).into_response()
    } else if p.is_dir() {
        let body = build_directory_list(&p, &state.path);
        (StatusCode::OK, Html(body)).into_response()
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content).into_response()
            }
            Err(e) => {
                warn!("Errorreading file: {:?}", e);
                (StatusCode::NOT_FOUND, Html("File not found".to_string())).into_response()
            }
        }
    }
}

fn build_directory_list(path: &std::path::Path, base_path: &PathBuf) -> String {
    let mut body = String::new();
    body.push_str("<html><body><ul>");

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let entry_path = entry.path();

        if entry_path.is_dir() {
            body.push_str(&format!("<li><a href=''>{}</a></li>", file_name));
            // 递归调用以构建子目录的列表
            let sub_list = build_directory_list(&entry_path, base_path);
            body.push_str(&sub_list);
        } else {
            // 计算相对路径
            let relative_path = entry_path
                .strip_prefix(base_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            body.push_str(&format!(
                "<li><a href='/{relative_path}'>{file_name}</a></li>"
            ));
        }
    }

    body.push_str("</ul></body></html>");
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        file_handler(State(state), Path("src".to_string())).await;
        // assert_eq!(, StatusCode::OK);
        // assert!(content.trim().starts_with("<!DOCTYPE html>"));
    }
}

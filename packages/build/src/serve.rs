use axum::Router;
use axum::routing::get;
use std::path::PathBuf;
use tower_http::services::ServeDir;

/// A simple dev server that serves the site root directory.
pub struct DevServer {
    pub site_root: PathBuf,
    pub addr: String,
}

impl DevServer {
    pub fn new(site_root: PathBuf, addr: &str) -> Self {
        Self {
            site_root,
            addr: addr.to_string(),
        }
    }

    /// Start the dev server. This blocks until the server stops.
    pub async fn run(&self) -> anyhow::Result<()> {
        let addr: std::net::SocketAddr = self.addr.parse()?;

        let app = Router::new()
            .fallback_service(
                ServeDir::new(&self.site_root)
                    .append_index_html_on_directories(true)
            );

        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("Dev server listening on http://{}", addr);

        axum::serve(listener, app).await?;

        Ok(())
    }
}
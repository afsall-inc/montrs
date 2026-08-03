use axum::Router;
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
    /// Tries to bind to the configured address, incrementing the port if in use.
    pub async fn run(&self) -> anyhow::Result<()> {
        let (host, port_str) = match self.addr.rsplit_once(':') {
            Some((h, p)) => (h, p),
            None => (self.addr.as_str(), "3000"),
        };
        let mut port: u16 = port_str.parse().unwrap_or(3000);

        let app = Router::new().fallback_service(
            ServeDir::new(&self.site_root)
                .append_index_html_on_directories(true),
        );

        let mut last_err = None;
        for _ in 0..100 {
            let bind_addr = format!("{host}:{port}");
            match tokio::net::TcpListener::bind(bind_addr.parse::<std::net::SocketAddr>()?).await {
                Ok(listener) => {
                    tracing::info!("Dev server listening on http://{host}:{port}");
                    axum::serve(listener, app).await?;
                    return Ok(());
                }
                Err(e) => {
                    last_err = Some(e);
                    port += 1;
                }
            }
        }

        anyhow::bail!(
            "Could not bind to any port in range {port_str}-{port}. Last error: {:?}",
            last_err
        )
    }
}

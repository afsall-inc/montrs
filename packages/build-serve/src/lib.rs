//! montrs-build-serve: Dev server for MontRS projects.
//!
//! Serves the site root directory and optionally spawns the SSR server.
//! Extracted from `montrs-build` to separate the HTTP serving concern
//! from the build pipeline.

#[cfg(test)]
pub mod test_helpers;

use anyhow::Result;
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::info;

/// Configuration for the dev server.
#[derive(Debug, Clone)]
pub struct ServeConfig {
    /// The address to bind to (e.g., "0.0.0.0:3000").
    pub addr: String,
    /// The root directory to serve files from.
    pub site_root: PathBuf,
    /// The WASM package directory relative to site_root.
    pub pkg_dir: PathBuf,
}

/// Start the static file dev server.
///
/// Serves files from `site_root` and logs the address. This is a
/// lightweight static file server — the SSR server binary is spawned
/// separately by the CLI.
pub async fn serve_static(config: ServeConfig) -> Result<()> {
    let app = Router::new()
        .fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev server listening on {}", config.addr);
    info!("Serving from {}", config.site_root.display());

    axum::serve(listener, app).await?;
    Ok(())
}

/// Start the dev server with a callback for when the server is ready.
pub async fn serve_with_callback<F>(config: ServeConfig, on_ready: F) -> Result<()>
where
    F: FnOnce(),
{
    let app = Router::new()
        .fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev server listening on {}", config.addr);
    on_ready();

    axum::serve(listener, app).await?;
    Ok(())
}
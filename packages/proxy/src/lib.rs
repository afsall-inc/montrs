//! montrs-proxy: Reverse proxy for local development.
//!
//! Routes `<slug>.localhost:<port>` to the appropriate service,
//! with optional TLS, mDNS, and port auto-detection.

pub mod server;
pub mod tls;

pub use server::{ProxyConfig, ProxyServer, RouteEntry};

/// Library-level error type.
#[derive(Debug, thiserror::Error)]
pub enum ProxyError {
    #[error("proxy server error: {0}")]
    Server(String),
    #[error("TLS error: {0}")]
    Tls(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("route not found: {0}")]
    RouteNotFound(String),
}

impl From<&str> for ProxyError {
    fn from(s: &str) -> Self {
        ProxyError::Server(s.to_string())
    }
}
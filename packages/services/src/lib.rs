//! montrs-services: Service supervisor for MontRS.
//!
//! Manages background services (daemons) with automatic start/stop,
//! ready checks, retry logic, lifecycle hooks, cron scheduling,
//! and file-watch-based restart. Designed after pitchfork.

pub mod config;
pub mod hooks;
pub mod ready;
pub mod retry;
pub mod service;
pub mod service_id;
pub mod state;
pub mod supervisor;

pub use config::ServiceConfig;
pub use service::Service;
pub use service_id::ServiceId;
pub use supervisor::Supervisor;

/// Library-level error type.
#[derive(Debug, thiserror::Error)]
pub enum ServicesError {
    #[error("service not found: {0}")]
    NotFound(ServiceId),
    #[error("service already running: {0}")]
    AlreadyRunning(ServiceId),
    #[error("service not running: {0}")]
    NotRunning(ServiceId),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("config error: {0}")]
    Config(String),
    #[error("state error: {0}")]
    State(String),
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("timeout waiting for {0}")]
    Timeout(String),
    #[error("{0}")]
    Other(String),
}

impl From<&str> for ServicesError {
    fn from(s: &str) -> Self {
        ServicesError::Other(s.to_string())
    }
}
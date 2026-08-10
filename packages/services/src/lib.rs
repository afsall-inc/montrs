pub mod config_types;
pub mod service_id;
pub mod supervisor;
pub mod ipc;
pub mod cli;

pub use service_id::{Service, ServiceId, ServiceState, ServiceStatus, ResourceLimits};

/// Errors from service operations.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Service '{0}' not found")]
    NotFound(String),
    #[error("Service '{0}' already running")]
    AlreadyRunning(String),
    #[error("Service '{0}' not running")]
    NotRunning(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Parse error: {0}")]
    Parse(String),
}
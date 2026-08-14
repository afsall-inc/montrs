//! montrs-log: Structured log store for MontRS services.
//!
//! Captures service output, supports line-based and structured (JSON/logfmt)
//! logs, streaming, retention, and archiving. This is the storage layer
//! powering `montrs services logs` and the TUI/web dashboards.

pub mod format;
pub mod store;

pub use format::{LogFormat, StructuredLog};
pub use store::{LogEntry, LogQuery, LogStore, LogStoreConfig, RetentionPolicy};

/// Library-level error type for the log store.
#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("log store I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("retention policy error: {0}")]
    Retention(String),
    #[error("service not found: {0}")]
    ServiceNotFound(String),
}
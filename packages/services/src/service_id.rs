//! ServiceId — a qualified identifier for a service (namespace/name).

use std::fmt;
use std::path::{Path, PathBuf};

/// A service identifier: `namespace/name` or just `name`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ServiceId {
    pub namespace: String,
    pub name: String,
}

impl ServiceId {
    pub fn new(namespace: &str, name: &str) -> Self {
        Self { namespace: namespace.to_string(), name: name.to_string() }
    }

    /// Parse from a string: `namespace/name` or just `name`.
    pub fn parse(s: &str) -> Self {
        if let Some((ns, n)) = s.split_once('/') {
            Self::new(ns, n)
        } else {
            Self::new("default", s)
        }
    }

    /// Full identifier string: `namespace/name`.
    pub fn full(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }

    /// Safe path encoding for filesystem paths.
    pub fn safe_path(&self) -> PathBuf {
        Path::new(&self.namespace).join(&self.name)
    }

    /// Display name for CLI output.
    pub fn display(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.namespace, self.name)
    }
}

impl From<&str> for ServiceId {
    fn from(s: &str) -> Self { Self::parse(s) }
}

// ---------------------------------------------------------------------------
// Service state
// ---------------------------------------------------------------------------

/// The current state of a service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
    Cron,
}

impl Default for ServiceState {
    fn default() -> Self { Self::Stopped }
}

/// Service status information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceStatus {
    pub id: ServiceId,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub uptime_ms: Option<u64>,
    pub port: Option<u16>,
    pub restart_count: u32,
    pub memory_kb: Option<u64>,
    pub cpu_percent: Option<f64>,
    pub started_at: Option<String>,
    pub error: Option<String>,
}

/// A running service instance.
#[derive(Debug, Clone)]
pub struct Service {
    pub id: ServiceId,
    pub command: String,
    pub args: Vec<String>,
    pub dir: Option<PathBuf>,
    pub env: std::collections::HashMap<String, String>,
    pub auto: bool,
    pub retry: super::config_types::Retry,
    pub stop_config: super::config_types::StopConfig,
    pub ready_http: Option<super::config_types::ReadyHttp>,
    pub ready_port: Option<super::config_types::ReadyPort>,
    pub ready_output: Option<super::config_types::ReadyOutput>,
    pub ready_cmd: Option<super::config_types::ReadyCmd>,
    pub hooks: super::config_types::Hooks,
    pub cron: Option<super::config_types::CronConfig>,
    pub watch_paths: Vec<PathBuf>,
    pub pty: bool,
    pub resource_limits: ResourceLimits,
}

/// Resource limits for a service.
#[derive(Debug, Clone, Default)]
pub struct ResourceLimits {
    pub cpu: Option<super::config_types::CpuLimit>,
    pub memory: Option<super::config_types::MemoryLimit>,
}
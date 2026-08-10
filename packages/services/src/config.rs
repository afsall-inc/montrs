//! Service configuration — parsed from the `[services]` section of `montrs.toml`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// A cron schedule definition for a service.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct CronSchedule {
    /// Cron expression (e.g., "0 */5 * * * *" for every 5 minutes).
    pub schedule: String,
    /// Whether to skip if the previous run is still in progress.
    #[serde(default)]
    pub skip_on_overlap: bool,
}

/// A ready-check method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReadyCheck {
    /// Wait N seconds before marking ready.
    Delay(u64),
    /// Match a regex pattern in stdout/stderr.
    Output(String),
    /// HTTP GET request to a URL succeeds.
    Http {
        url: String,
        #[serde(default = "default_ready_timeout")]
        timeout_secs: u64,
        #[serde(default)]
        expected_status: Option<u16>,
    },
    /// TCP port is open.
    Port {
        port: u16,
        #[serde(default = "default_ready_timeout")]
        timeout_secs: u64,
    },
    /// Run a custom command and check exit code.
    Cmd(String),
}

fn default_ready_timeout() -> u64 {
    30
}

/// Lifecycle hook definitions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct LifecycleHooks {
    /// Hook fired when the service becomes ready.
    pub on_ready: Option<String>,
    /// Hook fired when the service fails.
    pub on_fail: Option<String>,
    /// Hook fired on retry.
    pub on_retry: Option<String>,
    /// Hook fired when the service stops.
    pub on_stop: Option<String>,
    /// Hook fired when the service exits (even on failure).
    pub on_exit: Option<String>,
}

/// Resource limits for a service.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceLimits {
    /// Maximum memory in MB.
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU usage percentage.
    pub max_cpu_percent: Option<f64>,
}

/// Retry policy for a failing service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RetryPolicy {
    /// Maximum number of retries (default: 5).
    #[serde(default = "default_retry_count")]
    pub count: u32,
    /// Delay between retries in seconds (default: 1).
    #[serde(default = "default_retry_delay")]
    pub delay_secs: u64,
    /// Whether to use exponential backoff.
    #[serde(default)]
    pub backoff: bool,
    /// Maximum backoff delay in seconds (default: 60).
    #[serde(default = "default_max_backoff")]
    pub max_backoff_secs: u64,
}

fn default_retry_count() -> u32 {
    5
}
fn default_retry_delay() -> u64 {
    1
}
fn default_max_backoff() -> u64 {
    60
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            count: default_retry_count(),
            delay_secs: default_retry_delay(),
            backoff: false,
            max_backoff_secs: default_max_backoff(),
        }
    }
}

/// Auto-start/stop mode for shell hooks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AutoMode {
    #[default]
    None,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "start-stop")]
    StartStop,
}

impl AutoMode {
    pub fn should_start(&self) -> bool {
        matches!(self, AutoMode::Start | AutoMode::StartStop)
    }
    pub fn should_stop(&self) -> bool {
        matches!(self, AutoMode::Stop | AutoMode::StartStop)
    }
}

/// A single service configuration from the `[services]` section.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceConfig {
    /// The command to run (required).
    pub run: Option<String>,
    /// Arguments to the command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Working directory.
    pub dir: Option<PathBuf>,

    /// Auto start/stop behavior.
    #[serde(default)]
    pub auto: AutoMode,

    /// Dependencies — other service names that must start first.
    #[serde(default)]
    pub depends: Vec<String>,

    /// Environment variables.
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Cron schedule.
    pub cron: Option<CronSchedule>,

    /// Retry policy.
    #[serde(default)]
    pub retry: RetryPolicy,

    /// Ready checks.
    #[serde(default)]
    pub ready: Vec<ReadyCheck>,

    /// Delay before ready check (seconds).
    #[serde(default)]
    pub ready_delay: u64,

    /// Lifecycle hooks.
    #[serde(default)]
    pub hooks: LifecycleHooks,

    /// File patterns to watch for auto-restart.
    #[serde(default)]
    pub watch: Vec<String>,

    /// Resource limits.
    #[serde(default)]
    pub resource_limits: ResourceLimits,

    /// Allocate a PTY for the process.
    #[serde(default)]
    pub pty: bool,

    /// System user to run as.
    pub user: Option<String>,

    /// Tags for grouping/logging.
    #[serde(default)]
    pub tags: Vec<String>,

    /// Description of the service.
    pub description: Option<String>,
}

impl ServiceConfig {
    /// Parse from a raw TOML value (from the [services] section).
    pub fn from_toml_value(key: &str, value: &toml::Value) -> anyhow::Result<Self> {
        match value {
            toml::Value::String(cmd) => Ok(Self {
                run: Some(cmd.clone()),
                ..Default::default()
            }),
            toml::Value::Table(table) => {
                let val = toml::Value::Table(table.clone());
                let config: Self = val.try_into().map_err(|e| {
                    anyhow::anyhow!("failed to parse service '{key}': {e}")
                })?;
                if config.run.is_none() {
                    anyhow::bail!("service '{key}' must have a `run` field");
                }
                Ok(config)
            }
            _ => anyhow::bail!("invalid service config for '{key}'"),
        }
    }

    /// Parse an entire `[services]` section.
    pub fn from_toml_map(
        map: &std::collections::HashMap<String, toml::Value>,
    ) -> anyhow::Result<std::collections::HashMap<String, Self>> {
        let mut services = std::collections::HashMap::new();
        for (key, value) in map {
            let config = Self::from_toml_value(key, value)?;
            services.insert(key.clone(), config);
        }
        Ok(services)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string_config() {
        let value = toml::Value::String("redis-server".to_string());
        let config = ServiceConfig::from_toml_value("redis", &value).unwrap();
        assert_eq!(config.run, Some("redis-server".to_string()));
    }

    #[test]
    fn test_table_config() {
        let toml_str = r#"
run = "node server.js"
auto = "start-stop"
depends = ["redis"]
ready_delay = 3
[retry]
count = 3
backoff = true
"#;
        let value: toml::Value = toml::from_str(toml_str).unwrap();
        let config = ServiceConfig::from_toml_value("api", &value).unwrap();
        assert_eq!(config.run, Some("node server.js".to_string()));
        assert!(config.retry.backoff);
        assert_eq!(config.depends, vec!["redis"]);
    }

    #[test]
    fn test_auto_mode_parsing() {
        let toml_str = r#"
run = "cmd"
auto = "start"
"#;
        let value: toml::Value = toml::from_str(toml_str).unwrap();
        let config = ServiceConfig::from_toml_value("svc", &value).unwrap();
        assert!(config.auto.should_start());
        assert!(!config.auto.should_stop());
    }
}
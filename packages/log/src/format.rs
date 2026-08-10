//! Log format parsing and rendering.

use serde::{Deserialize, Serialize};

/// The on-disk / streaming format for a single log line.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum LogFormat {
    /// Plain text lines, one per message.
    #[default]
    Text,
    /// JSON object per line: `{"ts":..., "level":..., "msg":...}`.
    Json,
    /// logfmt: `ts=... level=info msg="..." field=value`.
    Logfmt,
}

impl LogFormat {
    /// Render a raw output line into a normalized structured record.
    pub fn render<'a>(
        &self,
        timestamp: &'a str,
        level: &'a str,
        service: &'a str,
        message: &'a str,
    ) -> String {
        match self {
            LogFormat::Text => format!("[{timestamp}] [{level}] {service}: {message}"),
            LogFormat::Json => {
                let rec = StructuredLog {
                    ts: timestamp.to_string(),
                    level: level.to_string(),
                    service: service.to_string(),
                    msg: message.to_string(),
                };
                serde_json::to_string(&rec).unwrap_or_else(|_| message.to_string())
            }
            LogFormat::Logfmt => {
                format!(
                    "ts={} level={} service={} msg=\"{}\"",
                    timestamp,
                    level,
                    service,
                    message.replace('"', "\\\"")
                )
            }
        }
    }

    /// Parse a JSON-structured log line into a record, if it is one.
    pub fn parse_json(line: &str) -> Option<StructuredLog> {
        serde_json::from_str::<StructuredLog>(line).ok()
    }
}

/// A structured (JSON) log record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLog {
    pub ts: String,
    pub level: String,
    pub service: String,
    pub msg: String,
}
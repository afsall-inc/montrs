//! Build configuration types.

use serde::{Deserialize, Serialize};

/// Build configuration for the MontRS build pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildConfig {
    /// Whether to build in release mode.
    #[serde(default)]
    pub release: bool,
    /// The build target (e.g., "index.html").
    #[serde(default = "default_target")]
    pub target: String,
    /// Whether to skip the frontend build.
    #[serde(default)]
    pub skip_frontend: bool,
    /// Whether to skip the server build.
    #[serde(default)]
    pub skip_server: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            release: false,
            target: default_target(),
            skip_frontend: false,
            skip_server: false,
        }
    }
}

fn default_target() -> String {
    "index.html".to_string()
}

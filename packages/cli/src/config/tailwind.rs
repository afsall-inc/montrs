use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TailwindToml {
    pub content: Option<Vec<String>>,
    pub theme: Option<serde_json::Value>,
    pub plugins: Option<Vec<String>>,
    pub merge: Option<MergeConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MergeConfig {
    pub prefix: Option<String>,
    pub separator: Option<String>,
}

impl TailwindToml {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}

/// No-op: Tailwind v4 uses CSS-only configuration (`@theme` blocks in CSS).
/// No JavaScript config file is ever generated.
pub fn ensure_tailwind_config(
    _project_root: &Path,
    _style: super::TailwindStyle,
) -> Result<Option<std::path::PathBuf>> {
    Ok(None)
}
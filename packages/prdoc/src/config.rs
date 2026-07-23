use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrdocConfig {
    #[serde(default)]
    pub generate: GenerateSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerateSection {
    #[serde(default)]
    pub default_output: String,
}

pub fn load_config(root: &Path) -> PrdocConfig {
    let config_path = root.join("prdoc.toml");
    if !config_path.exists() {
        return PrdocConfig::default();
    }

    let content = match std::fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => return PrdocConfig::default(),
    };

    toml::from_str(&content).unwrap_or_default()
}

pub fn find_project_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;
    loop {
        if current.join("Cargo.toml").exists()
            || current.join("prdoc.toml").exists()
        {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

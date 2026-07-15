#[cfg(feature = "llm")]
use crate::llm::LlmConfig;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrdocConfig {
    #[serde(default)]
    pub llm: LlmSection,
    #[serde(default)]
    pub generate: GenerateSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlmSection {
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerateSection {
    #[serde(default)]
    pub default_output: String,
}

impl PrdocConfig {
    #[cfg(feature = "llm")]
    pub fn to_llm_config(&self) -> Option<LlmConfig> {
        if self.llm.provider.is_empty() || self.llm.provider == "none" {
            return None;
        }

        let api_key = if !self.llm.api_key_env.is_empty() {
            std::env::var(&self.llm.api_key_env).unwrap_or_default()
        } else {
            String::new()
        };

        if api_key.is_empty()
            && self.llm.provider != "ollama"
            && self.llm.provider != "local"
        {
            return None;
        }

        Some(LlmConfig {
            provider: self.llm.provider.clone(),
            model: self.llm.model.clone(),
            api_key,
        })
    }

    #[cfg(not(feature = "llm"))]
    #[allow(dead_code)]
    pub fn to_llm_config(&self) -> Option<()> {
        None
    }
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

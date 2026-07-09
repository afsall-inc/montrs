use anyhow::{Context, Result, bail};
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
        let content =
            fs::read_to_string(path).context("Failed to read tailwind.toml")?;
        let config: Self = toml::from_str(&content)
            .context("Failed to parse tailwind.toml")?;
        Ok(config)
    }

    pub fn to_js(&self) -> String {
        let content = self
            .content
            .as_ref()
            .map(|c| serde_json::to_string(c).unwrap())
            .unwrap_or_else(|| "[]".to_string());
        let theme = self
            .theme
            .as_ref()
            .map(|t| serde_json::to_string_pretty(t).unwrap())
            .unwrap_or_else(|| "{}".to_string());

        let mut plugins_js = String::new();
        if let Some(plugins) = &self.plugins {
            for plugin in plugins {
                plugins_js.push_str(&format!("require('{}'),", plugin));
            }
        }

        format!(
            "module.exports = {{\n  content: {},\n  theme: {},\n  plugins: \
             [{}]\n}}",
            content, theme, plugins_js
        )
    }
}

pub fn ensure_tailwind_config(
    project_root: &Path,
    style: super::TailwindStyle,
) -> Result<Option<std::path::PathBuf>> {
    if matches!(style, super::TailwindStyle::V4) {
        return Ok(None);
    }

    let toml_path = project_root.join("tailwind.toml");

    // If Toml style is forced, or Auto and toml exists
    if matches!(style, super::TailwindStyle::Toml)
        || (matches!(style, super::TailwindStyle::Auto) && toml_path.exists())
    {
        if !toml_path.exists() && matches!(style, super::TailwindStyle::Toml) {
            bail!("tailwind.toml not found but --tailwind-toml was specified");
        }

        let config = TailwindToml::load(&toml_path)?;
        let js_content = config.to_js();
        let js_path = project_root.join("tailwind.config.js");

        fs::write(&js_path, js_content)
            .context("Failed to write tailwind.config.js")?;
        return Ok(Some(js_path));
    }
    Ok(None)
}

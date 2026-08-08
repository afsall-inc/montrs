//! Configuration plate for MontRS.

//! This plate defines the structure of the `montrs.toml` configuration file
//! and handles loading/parsing logic. It serves as the central source of truth
//! for project settings, build options, and server configuration.

use anyhow::{Context, Result};
use montrs_fmt::FormatterSettings;
use montrs_metadata::MontrsMetadata;
use serde::{Deserialize, Serialize};

/// The root configuration structure for a MontRS project.
///
/// Corresponds to the `montrs.toml` file. Delegates shared fields to
/// `MontrsMetadata` (the single source of truth), keeping only CLI-specific
/// configuration here.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MontrsConfig {
    /// Core metadata — single source of truth for project identity,
    /// serve, build, deploy, env, tasks, tools, etc.
    #[serde(flatten)]
    pub meta: MontrsMetadata,

    /// E2E testing configuration.
    #[serde(default)]
    pub e2e: E2eConfig,

    /// Formatting configuration.
    #[serde(default)]
    pub fmt: FormatterSettings,

    // Internal CLI fields (not serialized to montrs.toml)
    #[serde(skip)]
    pub verbose: u8,
    #[serde(skip)]
    pub log: Vec<String>,
    #[serde(skip)]
    pub release: bool,
    #[serde(skip)]
    pub hot_reload: bool,
    #[serde(skip)]
    pub features: Vec<String>,
}

/// E2E testing configuration.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct E2eConfig {
    /// Run browsers in headless mode.
    #[serde(default)]
    pub headless: Option<bool>,
    /// Browser to use (chromium, firefox, webkit).
    #[serde(default)]
    pub browser: Option<String>,
    /// Base URL for tests (overrides automatic detection).
    #[serde(default)]
    pub base_url: Option<String>,
}

impl MontrsConfig {
    /// Loads configuration from a specific file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let content =
            std::fs::read_to_string(path.as_ref()).with_context(|| {
                format!(
                    "Failed to read config file: {}",
                    path.as_ref().display()
                )
            })?;
        let mut config: Self = toml::from_str(&content).with_context(|| {
            format!("Failed to parse config file: {}", path.as_ref().display())
        })?;

        // Auto-detect project name from Cargo.toml if not set
        if config.meta.project.name.is_none()
            && let Ok(cargo) = cargo_metadata::MetadataCommand::new().exec()
            && let Some(root) = cargo.root_package()
        {
            config.meta.project.name = Some(root.name.clone());
        }

        Ok(config)
    }

    /// Loads configuration from `montrs.toml` in the current directory.
    ///
    /// If the file is missing, returns default configuration.
    /// Also attempts to resolve the project name from `Cargo.toml`.
    pub fn load() -> Result<Self> {
        let mut config = if std::path::Path::new("montrs.toml").exists() {
            Self::from_file("montrs.toml")?
        } else {
            Self::default()
        };

        // Cascade of Truth: Load montrs-fmt.toml if it exists and override the [fmt] section
        if std::path::Path::new("montrs-fmt.toml").exists() {
            let content = std::fs::read_to_string("montrs-fmt.toml")?;
            if let Ok(fmt_settings) = toml::from_str(&content) {
                config.fmt = fmt_settings;
            }
        }

        // Try to resolve project name if still default
        if config.meta.project.name.is_none()
            && let Ok(cargo) = cargo_metadata::MetadataCommand::new().exec()
            && let Some(root) = cargo.root_package()
        {
            config.meta.project.name = Some(root.name.clone());
        }

        Ok(config)
    }
}

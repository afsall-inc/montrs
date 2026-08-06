//! montrs-metadata: Project metadata abstraction for MontRS.
//!
//! Reads `montrs.toml` and provides all configuration needed for building,
//! serving, and deploying MontRS applications.
//!
//! # Example `montrs.toml`
//! ```toml
//! [project]
//! name = "my-app"
//!
//! [serve]
//! site-addr = "0.0.0.0:3000"
//! tailwind-input-file = "style/main.css"
//! site-root = "target/site"
//! site-pkg-dir = "pkg"
//! package = "app"
//! ```

#[cfg(test)]
pub mod test_helpers;

use serde::{Deserialize, Serialize};
use std::path::Path;

/// The full MontRS project metadata, read from `montrs.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MontrsMetadata {
    #[serde(default)]
    pub project: ProjectMeta,
    #[serde(default)]
    pub serve: ServeMeta,
    #[serde(default)]
    pub build: BuildMeta,
    #[serde(default)]
    pub tasks: std::collections::HashMap<String, toml::Value>,
}

/// Project identity metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMeta {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
}

/// Serve/build configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServeMeta {
    /// The single package name for both WASM and SSR.
    #[serde(default)]
    pub package: Option<String>,
    /// Output name for the WASM binary.
    #[serde(default)]
    pub output_name: Option<String>,
    /// Site address (default: "0.0.0.0:3000").
    #[serde(default = "default_site_addr")]
    pub site_addr: String,
    /// Port for the live reload WebSocket (default: 3001).
    #[serde(default = "default_reload_port")]
    pub reload_port: u16,
    /// Path to the site root directory (default: "target/site").
    #[serde(default = "default_site_root")]
    pub site_root: String,
    /// Path to the WASM package directory (default: "pkg").
    #[serde(default = "default_site_pkg_dir")]
    pub site_pkg_dir: String,
    /// Path to the Tailwind CSS input file.
    pub tailwind_input_file: Option<String>,
    /// Directory for static assets.
    pub assets_dir: Option<String>,
    /// Browser compatibility query (default: "defaults").
    #[serde(default = "default_browserquery")]
    pub browserquery: String,
    /// Features to enable for the WASM library.
    #[serde(default)]
    pub lib_features: Vec<String>,
    /// Whether to use default features for the WASM library.
    #[serde(default = "default_true")]
    pub lib_default_features: bool,
    /// Features to enable for the server binary.
    #[serde(default)]
    pub bin_features: Vec<String>,
    /// Whether to use default features for the server binary.
    #[serde(default = "default_true")]
    pub bin_default_features: bool,
    /// Whether to hash frontend files.
    #[serde(default)]
    pub hash_files: bool,
    /// Additional files to watch for changes.
    #[serde(default)]
    pub watch_additional_files: Vec<String>,
    /// Path to the style file.
    pub style_file: Option<String>,
}

impl Default for ServeMeta {
    fn default() -> Self {
        Self {
            package: None,
            output_name: None,
            site_addr: default_site_addr(),
            reload_port: default_reload_port(),
            site_root: default_site_root(),
            site_pkg_dir: default_site_pkg_dir(),
            tailwind_input_file: None,
            assets_dir: None,
            browserquery: default_browserquery(),
            lib_features: Vec::new(),
            lib_default_features: true,
            bin_features: Vec::new(),
            bin_default_features: true,
            hash_files: false,
            watch_additional_files: Vec::new(),
            style_file: None,
        }
    }
}

/// Build configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildMeta {
    #[serde(default)]
    pub release: bool,
    #[serde(default)]
    pub target: String,
}

impl Default for BuildMeta {
    fn default() -> Self {
        Self {
            release: false,
            target: "index.html".to_string(),
        }
    }
}

fn default_site_addr() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_reload_port() -> u16 {
    3001
}

fn default_site_root() -> String {
    "target/site".to_string()
}

fn default_site_pkg_dir() -> String {
    "pkg".to_string()
}

fn default_browserquery() -> String {
    "defaults".to_string()
}

fn default_true() -> bool {
    true
}

impl MontrsMetadata {
    /// Load metadata from a `montrs.toml` file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let mut meta: Self = toml::from_str(&content)?;

        // Auto-detect project name from Cargo.toml if not set
        if meta.project.name.is_none()
            && let Ok(cargo) = cargo_metadata::MetadataCommand::new().exec()
            && let Some(root) = cargo.root_package()
        {
            meta.project.name = Some(root.name.clone());
        }

        // If `package` is set, use it for both bin and lib discovery;
        // otherwise auto-discover from cargo metadata.
        let pkg_name = meta.serve.package.clone();

        if let Ok(cargo) = cargo_metadata::MetadataCommand::new().exec() {
            let project_path = path
                .as_ref()
                .canonicalize()
                .unwrap_or_default()
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default();

            for package in &cargo.packages {
                if let Some(pkg_path) = package.manifest_path.parent()
                    && !pkg_path.starts_with(&project_path)
                {
                    continue;
                }

                if let Some(ref name) = pkg_name {
                    if package.name == *name {
                        meta.serve.package = Some(package.name.clone());
                        break;
                    }
                } else {
                    let has_cdylib = package
                        .targets
                        .iter()
                        .any(|t| t.kind.iter().any(|k| k == "cdylib"));
                    let has_bin = package
                        .targets
                        .iter()
                        .any(|t| t.kind.iter().any(|k| k == "bin"));
                    if has_cdylib && has_bin {
                        meta.serve.package = Some(package.name.clone());
                        break;
                    }
                }
            }
        }

        Ok(meta)
    }
}

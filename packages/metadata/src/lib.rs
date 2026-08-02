//! montrs-metadata: Project metadata abstraction for MontRS.
//!
//! Reads `montrs.toml` and provides all configuration needed for building,
//! serving, and deploying MontRS applications — replacing the need for
//! `[[workspace.metadata.leptos]]` or `[package.metadata.leptos]`.
//!
//! # Example `montrs.toml`
//! ```toml
//! [project]
//! name = "my-app"
//!
//! [serve]
//! site-addr = "0.0.0.0:3000"
//! reload-port = 3001
//! tailwind-input-file = "style/main.css"
//! bin-package = "server"
//! lib-package = "app"
//! site-root = "target/site"
//! site-pkg-dir = "pkg"
//! lib-features = ["hydrate"]
//! ```

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
}

/// Project identity metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
            description: None,
        }
    }
}

/// Serve/build configuration equivalent to `[[workspace.metadata.leptos]]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeMeta {
    /// The binary package to build and run as the server.
    #[serde(default)]
    pub bin_package: Option<String>,
    /// The library package to compile to WASM.
    #[serde(default)]
    pub lib_package: Option<String>,
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
            bin_package: None,
            lib_package: None,
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
        if meta.project.name.is_none() {
            if let Ok(cargo) = cargo_metadata::MetadataCommand::new().exec() {
                if let Some(root) = cargo.root_package() {
                    meta.project.name = Some(root.name.clone());
                }
            }
        }

        // Auto-detect bin/lib packages from Cargo workspace if not set
        if meta.serve.bin_package.is_none() || meta.serve.lib_package.is_none() {
            if let Ok(cargo) = cargo_metadata::MetadataCommand::new().exec() {
                for package in &cargo.packages {
                    for target in &package.targets {
                        if target.kind.iter().any(|k| k == "bin") && meta.serve.bin_package.is_none() {
                            meta.serve.bin_package = Some(package.name.clone());
                        }
                        if target.kind.iter().any(|k| k == "cdylib") && meta.serve.lib_package.is_none() {
                            meta.serve.lib_package = Some(package.name.clone());
                        }
                    }
                }
            }
        }

        Ok(meta)
    }

    /// Generate the `[[workspace.metadata.leptos]]` JSON section from `montrs.toml`.
    ///
    /// This is injected into the workspace `Cargo.toml` before calling cargo-leptos.
    pub fn to_leptos_metadata_section(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();

        if let Some(name) = &self.project.name {
            map.insert("name".into(), serde_json::Value::String(name.clone()));
        }
        if let Some(bin) = &self.serve.bin_package {
            map.insert("bin-package".into(), serde_json::Value::String(bin.clone()));
        }
        if let Some(lib) = &self.serve.lib_package {
            map.insert("lib-package".into(), serde_json::Value::String(lib.clone()));
        }

        map.insert("site-addr".into(), serde_json::Value::String(self.serve.site_addr.clone()));
        map.insert("reload-port".into(), serde_json::Value::Number(self.serve.reload_port.into()));
        map.insert("site-root".into(), serde_json::Value::String(self.serve.site_root.clone()));
        map.insert("site-pkg-dir".into(), serde_json::Value::String(self.serve.site_pkg_dir.clone()));
        map.insert("browserquery".into(), serde_json::Value::String(self.serve.browserquery.clone()));
        map.insert("lib-default-features".into(), serde_json::Value::Bool(self.serve.lib_default_features));
        map.insert("bin-default-features".into(), serde_json::Value::Bool(self.serve.bin_default_features));
        map.insert("hash-files".into(), serde_json::Value::Bool(self.serve.hash_files));

        if let Some(tw) = &self.serve.tailwind_input_file {
            map.insert("tailwind-input-file".into(), serde_json::Value::String(tw.clone()));
        }
        if let Some(assets) = &self.serve.assets_dir {
            map.insert("assets-dir".into(), serde_json::Value::String(assets.clone()));
        }
        if !self.serve.lib_features.is_empty() {
            map.insert("lib-features".into(), serde_json::Value::Array(
                self.serve.lib_features.iter().map(|f| serde_json::Value::String(f.clone())).collect()
            ));
        }
        if !self.serve.bin_features.is_empty() {
            map.insert("bin-features".into(), serde_json::Value::Array(
                self.serve.bin_features.iter().map(|f| serde_json::Value::String(f.clone())).collect()
            ));
        }

        serde_json::Value::Object(map)
    }

/// Write the leptos metadata section to the workspace `Cargo.toml`.
    ///
    /// This ensures cargo-leptos can find the configuration without the user
    /// having to manually write `[[workspace.metadata.leptos]]`.
    pub fn inject_into_workspace_toml(&self) -> Result<(), anyhow::Error> {
        let path = Path::new("Cargo.toml");
        let content = std::fs::read_to_string(path)?;

        // Remove any existing leptos metadata sections
        let cleaned: Vec<&str> = content.lines()
            .filter(|l| !l.contains("metadata.leptos") && !l.starts_with("[[workspace.metadata"))
            .collect();

        // Generate the leptos section as TOML
        let section = self.to_leptos_metadata_section();
        let json_str = serde_json::to_string_pretty(&section)?;
        let toml_val: toml::Value = toml::from_str(&json_str)?;
        let toml_str = toml::to_string_pretty(&toml_val)?;

        let mut result = cleaned.join("\n");
        result.push_str("\n\n[[workspace.metadata.leptos]]\n");
        // toml_str has a top-level key "leptos" which we don't want
        let lines: Vec<&str> = toml_str.lines().filter(|l| !l.starts_with("[leptos]")).collect();
        result.push_str(&lines.join("\n"));
        result.push('\n');

        std::fs::write(path, result)?;
        Ok(())
    }
}
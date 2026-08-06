//! montrs-build-core: The MontRS build pipeline trait and configuration.
//!
//! Defines the `BuildPipeline` trait that orchestrates building the server,
//! frontend (WASM), Tailwind CSS, and asset copying. The concrete
//! `Pipeline` struct lives in `montrs-build` (the facade), but the trait
//! and its configuration types live here so that `montrs-build-watch` and
//! `montrs-build-serve` can depend on the interface without pulling in
//! heavy build-time dependencies.

pub mod config;

use anyhow::Result;
use montrs_metadata::MontrsMetadata;
use std::path::{Path, PathBuf};

/// A step in the build pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStep {
    Server,
    Frontend,
    Tailwind,
    Assets,
    IndexHtml,
}

/// The build pipeline trait.
///
/// Implementations orchestrate the full MontRS build process: compiling
/// the SSR server binary, building the WASM frontend, processing CSS,
/// copying assets, and generating the index.html.
pub trait BuildPipeline: Send + Sync {
    /// Build the SSR server binary.
    fn build_server(&self) -> Result<()>;

    /// Build the WASM frontend (with wasm-bindgen bundling).
    fn build_frontend(&self) -> Result<()>;

    /// Process Tailwind CSS if configured.
    fn process_tailwind(&self) -> Result<()>;

    /// Copy static assets to the site root.
    fn copy_assets(&self) -> Result<()>;

    /// Generate the index.html entry point.
    fn generate_index_html(&self) -> Result<()>;

    /// Run all build steps in order.
    fn build_all(&self) -> Result<()>;

    /// Returns the project metadata.
    fn metadata(&self) -> &MontrsMetadata;

    /// Returns the project root path.
    fn project_root(&self) -> &Path;

    /// Returns the site root (output) path.
    fn site_root(&self) -> &Path;

    /// Returns the WASM package output directory.
    fn pkg_dir(&self) -> &Path;
}

/// Build configuration extracted from project metadata.
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub project_root: PathBuf,
    pub metadata: MontrsMetadata,
}

impl ProjectConfig {
    pub fn from_root(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        let metadata = MontrsMetadata::from_file(root.join("montrs.toml"))?;
        Ok(Self {
            project_root: root,
            metadata,
        })
    }
}

/// Find the workspace target directory by walking up the tree.
pub fn find_workspace_target_dir(app_root: &Path) -> Result<PathBuf> {
    let mut current = app_root.to_path_buf();
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Ok(current.join("target"));
                }
            }
        }
        if !current.pop() {
            break;
        }
    }
    Ok(app_root.join("target"))
}
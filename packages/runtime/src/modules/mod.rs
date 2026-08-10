//! Module loader — trait for loading Rust/WASM modules into the runtime.

use async_trait::async_trait;
use std::path::Path;

/// A loaded module.
pub struct Module {
    pub name: String,
    pub code: Vec<u8>,
    pub kind: ModuleKind,
    pub source: ModuleSource,
}

pub enum ModuleKind {
    Rust,
    Wasm,
    Native,
}

pub enum ModuleSource {
    File(std::path::PathBuf),
    Inline(Vec<u8>),
    Network(String),
}

/// Trait for loading modules into the runtime.
#[async_trait]
pub trait ModuleLoader: Send + Sync {
    /// Resolve a module specifier to a path or URL.
    fn resolve(
        &self,
        specifier: &str,
        base: &Path,
    ) -> Result<String, anyhow::Error>;

    /// Load a module by its resolved specifier.
    async fn load(&self, specifier: &str) -> Result<Module, anyhow::Error>;

    /// Prepare to load a module (pre-processing).
    async fn prepare_load(
        &self,
        _specifier: &str,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

/// A simple file-based module loader.
pub struct FileModuleLoader {
    pub roots: Vec<std::path::PathBuf>,
}

#[async_trait]
impl ModuleLoader for FileModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        base: &Path,
    ) -> Result<String, anyhow::Error> {
        // Try relative to base first.
        let base_path = base.join(specifier);
        if base_path.exists() {
            return Ok(base_path.to_string_lossy().to_string());
        }
        // Try each root.
        for root in &self.roots {
            let path = root.join(specifier);
            if path.exists() {
                return Ok(path.to_string_lossy().to_string());
            }
        }
        Err(anyhow::anyhow!("module not found: {specifier}"))
    }

    async fn load(&self, specifier: &str) -> Result<Module, anyhow::Error> {
        let path = std::path::Path::new(specifier);
        if !path.exists() {
            return Err(anyhow::anyhow!("module not found: {specifier}"));
        }
        let code = tokio::fs::read(path).await?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("module")
            .to_string();
        let kind = if specifier.ends_with(".wasm") {
            ModuleKind::Wasm
        } else {
            ModuleKind::Native
        };
        Ok(Module {
            name,
            code,
            kind,
            source: ModuleSource::File(path.to_path_buf()),
        })
    }
}

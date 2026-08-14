/// Core backends — tools that have built-in installers (rustup, nvm, etc.).
use crate::backend::{BackendType, ToolBackend, ToolError, ToolVersion};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct CoreBackend {
    pub name: String,
    pub install_dir: PathBuf,
    pub default_version: String,
}

impl CoreBackend {
    pub fn new(name: &str, install_dir: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            install_dir,
            default_version: "latest".to_string(),
        }
    }
}

#[async_trait]
impl ToolBackend for CoreBackend {
    fn name(&self) -> &str {
        &self.name
    }
    fn backend_type(&self) -> BackendType {
        BackendType::Core
    }
    fn install_dir(&self) -> PathBuf {
        self.install_dir.clone()
    }

    async fn list_versions(&self) -> Result<Vec<String>, ToolError> {
        // Core tools delegate to their native installers — return a placeholder.
        Ok(vec!["latest".to_string()])
    }

    fn list_installed(&self) -> Result<Vec<String>, ToolError> {
        let mut versions = Vec::new();
        if !self.install_dir.exists() {
            return Ok(versions);
        }
        for entry in std::fs::read_dir(&self.install_dir)? {
            let entry = entry?;
            if entry.path().is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                versions.push(name.to_string());
            }
        }
        Ok(versions)
    }

    async fn install(&self, version: &str) -> Result<ToolVersion, ToolError> {
        let version_path = self.version_path(version);
        tokio::fs::create_dir_all(&version_path).await?;
        Ok(ToolVersion {
            name: self.name.clone(),
            version: version.to_string(),
            backend: BackendType::Core,
            url: None,
            checksum: None,
            install_path: version_path,
            bins: vec![self.name.clone()],
        })
    }

    fn uninstall(&self, version: &str) -> Result<(), ToolError> {
        let path = self.version_path(version);
        if !path.exists() {
            return Err(ToolError::NotInstalled(format!(
                "{}@{}",
                self.name, version
            )));
        }
        std::fs::remove_dir_all(&path)?;
        Ok(())
    }

    fn is_installed(&self, version: &str) -> bool {
        self.version_path(version).exists()
    }
}

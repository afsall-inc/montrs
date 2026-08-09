/// Cargo backend — installs Rust tools via `cargo install`.
use crate::backend::{BackendType, ToolBackend, ToolError, ToolVersion};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct CargoBackend {
    pub name: String,
    pub install_dir: PathBuf,
}

impl CargoBackend {
    pub fn new(name: &str, install_dir: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            install_dir,
        }
    }
}

#[async_trait]
impl ToolBackend for CargoBackend {
    fn name(&self) -> &str {
        &self.name
    }
    fn backend_type(&self) -> BackendType {
        BackendType::Cargo
    }
    fn install_dir(&self) -> PathBuf {
        self.install_dir.clone()
    }

    async fn list_versions(&self) -> Result<Vec<String>, ToolError> {
        // Query crates.io for versions.
        let url = format!("https://crates.io/api/v1/crates/{}", self.name);
        let client = reqwest::Client::builder()
            .user_agent("montrs/0.1.0")
            .build()?;
        let response = client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;
        let versions: Vec<String> = data
            .get("versions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.get("num").and_then(|n| n.as_str()))
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();
        Ok(versions)
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

        // Run cargo install into a temp bin dir, then copy.
        let bin_dir = version_path.join("bin");
        tokio::fs::create_dir_all(&bin_dir).await?;

        let ver_arg = if version == "latest" {
            String::new()
        } else {
            format!("--version={version}")
        };
        let mut args = vec!["install".to_string(), self.name.clone()];
        if !ver_arg.is_empty() {
            args.push(ver_arg);
        }
        args.push("--root".to_string());
        args.push(version_path.to_string_lossy().to_string());

        let status = tokio::process::Command::new("cargo")
            .args(&args)
            .status()
            .await
            .map_err(|e| ToolError::Backend(e.to_string()))?;

        if !status.success() {
            return Err(ToolError::Backend(format!(
                "cargo install failed for {}",
                self.name
            )));
        }

        Ok(ToolVersion {
            name: self.name.clone(),
            version: version.to_string(),
            backend: BackendType::Cargo,
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

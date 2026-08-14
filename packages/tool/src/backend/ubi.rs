/// Ubi backend — universal binary installer.
use crate::backend::{
    BackendType, ToolBackend, ToolError, ToolVersion, download_file,
    extract_tarball, sha256_digest,
};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct UbiBackend {
    pub name: String,
    pub install_dir: PathBuf,
}

impl UbiBackend {
    pub fn new(name: &str, install_dir: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            install_dir,
        }
    }
}

#[async_trait]
impl ToolBackend for UbiBackend {
    fn name(&self) -> &str {
        &self.name
    }
    fn backend_type(&self) -> BackendType {
        BackendType::Ubi
    }
    fn install_dir(&self) -> PathBuf {
        self.install_dir.clone()
    }

    async fn list_versions(&self) -> Result<Vec<String>, ToolError> {
        // Query GitHub releases for the ubi-style tool.
        let url =
            format!("https://api.github.com/repos/{}/releases", self.name);
        let client = reqwest::Client::builder()
            .user_agent("montrs/0.1.0")
            .build()?;
        let response = client.get(&url).send().await?;
        let releases: Vec<serde_json::Value> = response.json().await?;
        let mut versions: Vec<String> = releases
            .iter()
            .filter_map(|r| r.get("tag_name").and_then(|t| t.as_str()))
            .map(|t| t.trim_start_matches('v').to_string())
            .filter(|v| {
                !v.contains("alpha") && !v.contains("beta") && !v.contains("rc")
            })
            .collect();
        versions.sort_by(|a, b| b.cmp(a));
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
        if version_path.exists() {
            return Err(ToolError::AlreadyInstalled(format!(
                "{}@{}",
                self.name, version
            )));
        }

        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let ext = if cfg!(windows) { "zip" } else { "tar.gz" };
        let asset =
            format!("{name}-{version}-{os}-{arch}.{ext}", name = self.name);
        let url = format!(
            "https://github.com/{}/releases/download/v{version}/{asset}",
            self.name
        );

        let archive_path = self.install_dir.join(&asset);
        download_file(&url, &archive_path).await?;
        tokio::fs::create_dir_all(&version_path).await?;
        extract_tarball(&archive_path, &version_path).await?;

        let checksum = sha256_digest(&archive_path).await?;
        let _ = tokio::fs::remove_file(&archive_path).await;

        Ok(ToolVersion {
            name: self.name.clone(),
            version: version.to_string(),
            backend: BackendType::Ubi,
            url: Some(url),
            checksum: Some(checksum),
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

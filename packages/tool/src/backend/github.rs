/// GitHub releases backend — downloads from GitHub releases.
use crate::backend::{
    BackendType, ToolBackend, ToolError, ToolVersion, download_file,
    extract_tarball, sha256_digest,
};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct GitHubBackend {
    pub name: String,
    pub repo: String,
    pub install_dir: PathBuf,
    pub asset_pattern: String,
}

impl GitHubBackend {
    pub fn new(name: &str, repo: &str, install_dir: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            repo: repo.to_string(),
            install_dir,
            asset_pattern: format!("{name}-{{version}}-{{os}}-{{arch}}.tar.gz"),
        }
    }

    fn api_url(&self) -> String {
        format!("https://api.github.com/repos/{}/releases", self.repo)
    }

    fn asset_url(&self, version: &str, asset: &str) -> String {
        format!(
            "https://github.com/{}/releases/download/{version}/{asset}",
            self.repo
        )
    }

    fn resolve_asset_name(&self, version: &str) -> String {
        let os = std::env::consts::OS;
        let arch = match std::env::consts::ARCH {
            "x86_64" => "x86_64",
            "aarch64" => "aarch64",
            _ => "x86_64",
        };
        self.asset_pattern
            .replace("{version}", version)
            .replace("{os}", os)
            .replace("{arch}", arch)
    }
}

#[async_trait]
impl ToolBackend for GitHubBackend {
    fn name(&self) -> &str {
        &self.name
    }
    fn backend_type(&self) -> BackendType {
        BackendType::GitHub
    }
    fn install_dir(&self) -> PathBuf {
        self.install_dir.clone()
    }

    async fn list_versions(&self) -> Result<Vec<String>, ToolError> {
        let client = reqwest::Client::builder()
            .user_agent("montrs/0.1.0")
            .build()?;
        let response = client.get(self.api_url()).send().await?;
        let releases: Vec<serde_json::Value> = response.json().await?;
        let mut versions: Vec<String> = releases
            .iter()
            .filter_map(|r| r.get("tag_name").and_then(|t| t.as_str()))
            .map(|t| t.trim_start_matches('v').to_string())
            .filter(|v| {
                !v.contains("alpha")
                    && !v.contains("beta")
                    && !v.contains("rc")
                    && !v.contains("nightly")
            })
            .collect();
        versions.sort_by(|a, b| {
            let a_parts: Vec<&str> = a.split('.').collect();
            let b_parts: Vec<&str> = b.split('.').collect();
            a_parts.cmp(&b_parts)
        });
        versions.reverse();
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

        let asset = self.resolve_asset_name(version);
        let url = self.asset_url(version, &asset);
        let archive_path = self.install_dir.join(format!("{}.tar.gz", version));

        download_file(&url, &archive_path).await?;
        tokio::fs::create_dir_all(&version_path).await?;
        extract_tarball(&archive_path, &version_path).await?;

        let checksum = sha256_digest(&archive_path).await?;
        let _ = tokio::fs::remove_file(&archive_path).await;

        Ok(ToolVersion {
            name: self.name.clone(),
            version: version.to_string(),
            backend: BackendType::GitHub,
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

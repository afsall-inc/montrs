/// HTTP backend — downloads from arbitrary HTTP URLs.
use crate::backend::{
    BackendType, ToolBackend, ToolError, ToolVersion, download_file,
    extract_tarball, sha256_digest,
};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct HttpBackend {
    pub name: String,
    pub install_dir: PathBuf,
    pub url_template: String,
}

impl HttpBackend {
    pub fn new(name: &str, install_dir: PathBuf, url_template: &str) -> Self {
        Self {
            name: name.to_string(),
            install_dir,
            url_template: url_template.to_string(),
        }
    }

    fn resolve_url(&self, version: &str) -> String {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        self.url_template
            .replace("{version}", version)
            .replace("{os}", os)
            .replace("{arch}", arch)
    }
}

#[async_trait]
impl ToolBackend for HttpBackend {
    fn name(&self) -> &str {
        &self.name
    }
    fn backend_type(&self) -> BackendType {
        BackendType::Http
    }
    fn install_dir(&self) -> PathBuf {
        self.install_dir.clone()
    }

    async fn list_versions(&self) -> Result<Vec<String>, ToolError> {
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
        if version_path.exists() {
            return Err(ToolError::AlreadyInstalled(format!(
                "{}@{}",
                self.name, version
            )));
        }

        let url = self.resolve_url(version);
        let archive_path = self
            .install_dir
            .join(format!("{}-{}.tar.gz", self.name, version));
        download_file(&url, &archive_path).await?;
        tokio::fs::create_dir_all(&version_path).await?;
        extract_tarball(&archive_path, &version_path).await?;

        let checksum = sha256_digest(&archive_path).await?;
        let _ = tokio::fs::remove_file(&archive_path).await;

        Ok(ToolVersion {
            name: self.name.clone(),
            version: version.to_string(),
            backend: BackendType::Http,
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

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod cargo;
pub mod core;
pub mod github;
pub mod http;
pub mod standalone;
pub mod ubi;

use async_trait::async_trait;
use montrs_registry::RegistryTool;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The type of backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendType {
    Core,
    Cargo,
    GitHub,
    Go,
    Http,
    Asdf,
    Vfox,
    Ubi,
    Aqua,
    Pipx,
    Npm,
    Gem,
}

impl BackendType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Cargo => "cargo",
            Self::GitHub => "github",
            Self::Go => "go",
            Self::Http => "http",
            Self::Asdf => "asdf",
            Self::Vfox => "vfox",
            Self::Ubi => "ubi",
            Self::Aqua => "aqua",
            Self::Pipx => "pipx",
            Self::Npm => "npm",
            Self::Gem => "gem",
        }
    }
}

/// A resolved tool version with download info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolVersion {
    pub name: String,
    pub version: String,
    pub backend: BackendType,
    pub url: Option<String>,
    pub checksum: Option<String>,
    pub install_path: PathBuf,
    pub bins: Vec<String>,
}

/// Errors from tool operations.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Tool '{0}' not found in registry")]
    NotFound(String),
    #[error("Version '{0}' not found for tool '{1}'")]
    VersionNotFound(String, String),
    #[error("Already installed: {0}")]
    AlreadyInstalled(String),
    #[error("Not installed: {0}")]
    NotInstalled(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Backend error: {0}")]
    Backend(String),
    #[error("Checksum mismatch for {0}")]
    ChecksumMismatch(String),
}

/// The backend trait — how to install, list versions, and resolve a tool.
#[async_trait]
pub trait ToolBackend: Send + Sync {
    fn name(&self) -> &str;
    fn backend_type(&self) -> BackendType;
    fn install_dir(&self) -> PathBuf;
    async fn list_versions(&self) -> Result<Vec<String>, ToolError>;
    fn list_installed(&self) -> Result<Vec<String>, ToolError>;
    async fn install(&self, version: &str) -> Result<ToolVersion, ToolError>;
    fn uninstall(&self, version: &str) -> Result<(), ToolError>;
    fn is_installed(&self, version: &str) -> bool;
    fn version_path(&self, version: &str) -> PathBuf {
        self.install_dir().join(version)
    }
}

/// Default install directory for tools.
pub fn default_install_dir() -> PathBuf {
    dirs::data_dir()
        .map(|d| d.join("montrs").join("installs"))
        .unwrap_or_else(|| PathBuf::from(".montrs/installs"))
}

/// Default shims directory.
pub fn default_shims_dir() -> PathBuf {
    dirs::data_dir()
        .map(|d| d.join("montrs").join("shims"))
        .unwrap_or_else(|| PathBuf::from(".montrs/shims"))
}

/// Compute SHA256 checksum of a file.
pub async fn sha256_digest(
    path: &std::path::Path,
) -> Result<String, ToolError> {
    use sha2::{Digest, Sha256};
    use tokio::io::AsyncReadExt;
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

/// Download a file from a URL to a path.
pub async fn download_file(
    url: &str,
    dest: &std::path::Path,
) -> Result<(), ToolError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(dest, &bytes).await?;
    Ok(())
}

/// Extract a tarball to a directory.
pub async fn extract_tarball(
    archive: &std::path::Path,
    dest: &std::path::Path,
) -> Result<(), ToolError> {
    use tokio::io::AsyncReadExt;
    let mut file = tokio::fs::File::open(archive).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let decoder = flate2::read::GzDecoder::new(&buf[..]);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(dest)?;
    Ok(())
}

/// Create the appropriate backend for a tool based on registry info.
pub fn create_backend(
    name: &str,
    backend_type: &str,
    install_dir: Option<PathBuf>,
    tool: Option<&RegistryTool>,
) -> Result<Box<dyn ToolBackend>, ToolError> {
    let dir = install_dir.unwrap_or_else(|| default_install_dir().join(name));
    let parts: Vec<&str> = backend_type.split(':').collect();
    let backend = parts.first().copied().unwrap_or("");

    match backend {
        "core" => Ok(Box::new(core::CoreBackend::new(name, dir))),
        "cargo" => {
            // Use parts[1] as crate name (e.g. "cargo:wasm-bindgen-cli"),
            // fall back to tool name.
            let crate_name = parts.get(1).copied().unwrap_or(name);
            Ok(Box::new(cargo::CargoBackend::new(crate_name, dir)))
        }
        "github" => {
            let repo = parts.get(1).copied().unwrap_or(name);
            let is_standalone = parts.get(2) == Some(&"standalone")
                || tool.map(|t| t.option_bool("standalone")).unwrap_or(false);
            if is_standalone {
                let asset =
                    tool.and_then(|t| t.option_str("asset")).unwrap_or_else(
                        || format!("{name}-{{os}}-{{arch}}{{exe}}"),
                    );
                Ok(Box::new(standalone::StandaloneBackend::new(
                    name, repo, dir, asset,
                )))
            } else {
                Ok(Box::new(github::GitHubBackend::new(name, repo, dir)))
            }
        }
        "http" => Ok(Box::new(http::HttpBackend::new(name, dir, ""))),
        "ubi" => Ok(Box::new(ubi::UbiBackend::new(name, dir))),
        _ => Ok(Box::new(github::GitHubBackend::new(name, name, dir))),
    }
}

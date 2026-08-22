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

/// Standalone backend — downloads a raw executable from GitHub releases.
/// Used for tools distributed as single binaries (e.g. Tailwind CSS CLI),
/// avoiding any dependency on npm/Node.
use crate::backend::{
    BackendType, ToolBackend, ToolError, ToolVersion, candidate_tags,
    download_file, http_get_with_retry, sha256_digest,
};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct StandaloneBackend {
    pub name: String,
    pub repo: String,
    pub install_dir: PathBuf,
    /// Asset name template, e.g. "tailwindcss-{os}-{arch}{exe}".
    pub asset_template: String,
}

impl StandaloneBackend {
    pub fn new(
        name: &str,
        repo: &str,
        install_dir: PathBuf,
        asset_template: String,
    ) -> Self {
        Self {
            name: name.to_string(),
            repo: repo.to_string(),
            install_dir,
            asset_template,
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

    /// Resolve the asset file name for the current platform.
    fn resolve_asset_name(&self, version: &str) -> String {
        let os = std::env::consts::OS;
        let arch = match std::env::consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            other => other,
        };
        let exe = if cfg!(windows) { ".exe" } else { "" };
        self.asset_template
            .replace("{version}", version)
            .replace("{os}", os)
            .replace("{arch}", arch)
            .replace("{exe}", exe)
    }
}

#[async_trait]
impl ToolBackend for StandaloneBackend {
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
        let response = http_get_with_retry(&self.api_url(), 3).await?;
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
        let bin_dir = version_path.join("bin");
        let download_path = bin_dir.join(if cfg!(windows) {
            format!("{}.exe", self.name)
        } else {
            self.name.clone()
        });

        // Try each tag form (e.g. "4.3.1", then "v4.3.1") until one
        // downloads successfully. See `candidate_tags` for details.
        let mut last_error: Option<String> = None;
        let mut downloaded_url = String::new();
        for tag in candidate_tags(version) {
            let url = self.asset_url(&tag, &asset);
            match download_file(&url, &download_path).await {
                Ok(()) => {
                    downloaded_url = url;
                    break;
                }
                Err(e) => last_error = Some(e.to_string()),
            }
        }
        if downloaded_url.is_empty() {
            return Err(ToolError::Backend(format!(
                "failed to download {}@{}{}",
                self.name,
                version,
                last_error.map(|e| format!(": {e}")).unwrap_or_default(),
            )));
        }

        // Mark executable on Unix so it can be invoked directly.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&download_path, perms)?;
        }

        let checksum = sha256_digest(&download_path).await?;

        Ok(ToolVersion {
            name: self.name.clone(),
            version: version.to_string(),
            backend: BackendType::GitHub,
            url: Some(downloaded_url),
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

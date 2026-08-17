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

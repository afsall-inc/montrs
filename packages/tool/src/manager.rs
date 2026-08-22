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

use crate::backend::create_backend;
/// ToolManager — orchestrates tool installation, resolution, and management.
use crate::backend::{
    ToolBackend, ToolError, ToolVersion, default_install_dir, default_shims_dir,
};
use montrs_lockfile::{lockfile_path_for_root, read_lockfile};
use montrs_registry::{BAKED_REGISTRY, RegistryTool};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// A tool request: name + optional version.
#[derive(Debug, Clone)]
pub struct ToolRequest {
    pub name: String,
    pub version: Option<String>,
}

impl ToolRequest {
    pub fn parse(spec: &str) -> Self {
        if let Some((name, version)) = spec.split_once('@') {
            Self {
                name: name.to_string(),
                version: Some(version.to_string()),
            }
        } else {
            Self {
                name: spec.to_string(),
                version: None,
            }
        }
    }
}

/// The main tool manager.
pub struct ToolManager {
    pub install_dir: PathBuf,
    pub shims_dir: PathBuf,
    pub tools: HashMap<String, RegistryTool>,
}

impl ToolManager {
    pub fn new() -> Self {
        Self {
            install_dir: default_install_dir(),
            shims_dir: default_shims_dir(),
            tools: BAKED_REGISTRY.tools.clone(),
        }
    }

    pub fn with_dirs(install_dir: PathBuf, shims_dir: PathBuf) -> Self {
        Self {
            install_dir,
            shims_dir,
            tools: BAKED_REGISTRY.tools.clone(),
        }
    }

    /// Look up a tool in the registry.
    pub fn lookup(&self, name: &str) -> Option<&RegistryTool> {
        self.tools.get(name)
    }

    /// Create a backend for a tool.
    pub fn backend_for(
        &self,
        name: &str,
    ) -> Result<Box<dyn ToolBackend>, ToolError> {
        let tool = self
            .lookup(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?;
        let backend_str = tool
            .backends
            .first()
            .map(|s| s.as_str())
            .unwrap_or("github");
        create_backend(
            name,
            backend_str,
            Some(self.install_dir.join(name)),
            Some(tool),
        )
    }

    /// Install a tool at a specific (or latest) version.
    /// If version is "latest", resolves the latest stable version from the backend.
    /// If version is a semver range (e.g. "4", "0.2", "^0.2"), resolves it to the
    /// latest concrete version from the backend that matches.
    pub async fn install(
        &self,
        request: &ToolRequest,
    ) -> Result<ToolVersion, ToolError> {
        let backend = self.backend_for(&request.name)?;
        let version = match &request.version {
            Some(v) if v != "latest" => {
                let parts: Vec<&str> =
                    v.trim_start_matches(&['^', '~']).split('.').collect();
                // Resolve ranges like "4" or "0.2" to the latest matching
                // concrete version ("4.3.1", "0.2.101"); exact versions pass
                // through unchanged.
                if parts.len() < 3 {
                    let prefix = format!("{}.", parts.join("."));
                    let all_versions = backend.list_versions().await?;
                    all_versions
                        .iter()
                        .find(|ver| ver.starts_with(&prefix))
                        .cloned()
                        .unwrap_or_else(|| v.clone())
                } else {
                    v.clone()
                }
            }
            _ => {
                let versions = backend.list_versions().await?;
                versions
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "latest".to_string())
            }
        };
        backend.install(&version).await
    }

    /// List installed versions of a tool.
    pub fn list_installed(&self, name: &str) -> Result<Vec<String>, ToolError> {
        let backend = self.backend_for(name)?;
        backend.list_installed()
    }

    /// List all available versions of a tool.
    pub async fn list_remote(
        &self,
        name: &str,
    ) -> Result<Vec<String>, ToolError> {
        let backend = self.backend_for(name)?;
        backend.list_versions().await
    }

    /// Uninstall a specific version.
    pub fn uninstall(
        &self,
        name: &str,
        version: &str,
    ) -> Result<(), ToolError> {
        let backend = self.backend_for(name)?;
        backend.uninstall(version)
    }

    /// Check if a version is installed.
    pub fn is_installed(
        &self,
        name: &str,
        version: &str,
    ) -> Result<bool, ToolError> {
        let backend = self.backend_for(name)?;
        Ok(backend.is_installed(version))
    }

    /// Get the path where a tool version is installed.
    pub fn version_path(
        &self,
        name: &str,
        version: &str,
    ) -> Result<PathBuf, ToolError> {
        let backend = self.backend_for(name)?;
        Ok(backend.version_path(version))
    }

    /// Whether a tool is installed at all.
    pub fn tool_is_installed(&self, name: &str) -> Result<bool, ToolError> {
        let versions = self.list_installed(name)?;
        Ok(!versions.is_empty())
    }

    /// Create a shim for a tool's binary.
    pub fn create_shim(
        &self,
        name: &str,
        bin: &str,
        version: &str,
    ) -> Result<(), ToolError> {
        std::fs::create_dir_all(&self.shims_dir)?;
        let shim_path = self.shims_dir.join(bin);
        let install_path = self.version_path(name, version)?;
        let target = install_path.join("bin").join(bin);

        if cfg!(windows) {
            // Windows: copy the binary.
            if target.exists() {
                std::fs::copy(&target, &shim_path)?;
            }
        } else {
            // Unix: create an exec wrapper script.
            let script = format!(
                "#!/bin/sh\nexec \"{}\" \"$@\"\n",
                target.to_string_lossy()
            );
            std::fs::write(&shim_path, script)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    &shim_path,
                    std::fs::Permissions::from_mode(0o755),
                )?;
            }
        }
        Ok(())
    }

    /// Get the current default version of an installed tool.
    pub fn current(&self, name: &str) -> Result<Option<String>, ToolError> {
        let versions = self.list_installed(name)?;
        Ok(versions.first().cloned())
    }
}

/// Resolve the absolute path to an installed, managed binary.
///
/// Looks up the version from `montrs.lock` first (so `montrs install`
/// and the build pipeline agree on versions), then falls back to any
/// installed version in the shared install directory.
pub fn managed_bin_path(
    name: &str,
    bin_name: &str,
    project_root: &Path,
) -> Option<PathBuf> {
    let install_root = default_install_dir().join(name);
    let lock_path = lockfile_path_for_root(project_root);
    let locked_version = read_lockfile(&lock_path).ok().and_then(|lock| {
        lock.resolved_version(name).map(|tool| tool.version.clone())
    });

    let version = locked_version.or_else(|| {
        std::fs::read_dir(&install_root)
            .ok()?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .find(|v| !v.is_empty())
    })?;

    let bin_dir = install_root.join(version).join("bin");
    if cfg!(windows) {
        let exe = bin_dir.join(format!("{bin_name}.exe"));
        if exe.exists() {
            return Some(exe);
        }
    }
    let bin = bin_dir.join(bin_name);
    if bin.exists() {
        return Some(bin);
    }
    None
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}

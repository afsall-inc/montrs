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

/// MontRS plugin system — asdf-compatible tool plugins.
///
/// Ported/adapted from mise's plugin system, tailored for MontRS.
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The type of plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    /// git-based asdf plugin
    Asdf,
    /// Lua-based vfox plugin
    Vfox,
}

/// A plugin source (where to install the plugin from).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginSource {
    /// A git repository URL.
    Git(String),
    /// A local directory path.
    Local(String),
    /// A zip archive URL.
    Zip(String),
}

impl PluginSource {
    pub fn is_git(&self) -> bool {
        matches!(self, PluginSource::Git(_))
    }
    pub fn is_local(&self) -> bool {
        matches!(self, PluginSource::Local(_))
    }
    pub fn is_zip(&self) -> bool {
        matches!(self, PluginSource::Zip(_))
    }
    pub fn as_str(&self) -> &str {
        match self {
            PluginSource::Git(s)
            | PluginSource::Local(s)
            | PluginSource::Zip(s) => s,
        }
    }
}

/// Errors from plugin operations.
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin '{0}' not installed")]
    NotInstalled(String),
    #[error("Plugin '{0}' already installed")]
    AlreadyInstalled(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git error: {0}")]
    Git(String),
    #[error("Plugin '{0}' not found in registry")]
    NotFoundInRegistry(String),
}

/// A plugin — manages a single tool's installation lifecycle.
#[async_trait]
pub trait Plugin: Send + Sync {
    /// The plugin's short name (e.g. "rust", "node").
    fn name(&self) -> &str;
    /// The plugin type.
    fn plugin_type(&self) -> PluginType;
    /// Directory where the plugin's scripts live.
    fn plugin_path(&self) -> &Path;
    /// List available versions for an installed plugin.
    async fn list_versions(
        &self,
        prefix: Option<&str>,
    ) -> anyhow::Result<Vec<String>>;
    /// List installed versions.
    async fn list_installed(&self) -> anyhow::Result<Vec<String>>;
    /// Install a specific version.
    async fn install(&self, version: &str) -> anyhow::Result<()>;
    /// Uninstall a specific version.
    async fn uninstall(&self, version: &str) -> anyhow::Result<()>;
    /// Whether a version is installed.
    async fn is_installed(&self, version: &str) -> anyhow::Result<bool>;
}

/// A plugin registered in the system.
#[derive(Debug, Clone)]
pub struct PluginRecord {
    pub name: String,
    pub plugin_type: PluginType,
    pub source: Option<PluginSource>,
    pub path: PathBuf,
}

/// The plugin registry — tracks known plugins and their locations.
#[derive(Debug, Default)]
pub struct PluginRegistry {
    pub plugins_dir: PathBuf,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let plugins_dir = dirs::data_dir()
            .map(|d| d.join("montrs").join("plugins"))
            .unwrap_or_else(|| PathBuf::from(".montrs/plugins"));
        Self { plugins_dir }
    }

    pub fn with_plugins_dir(dir: PathBuf) -> Self {
        Self { plugins_dir: dir }
    }

    /// Path where a plugin would be installed.
    pub fn plugin_path(&self, name: &str) -> PathBuf {
        self.plugins_dir.join(name)
    }

    /// Whether a plugin is installed.
    pub fn is_installed(&self, name: &str) -> bool {
        self.plugin_path(name).exists()
    }

    /// List all installed plugins.
    pub fn list_installed(&self) -> Vec<PluginRecord> {
        let mut plugins = Vec::new();
        if !self.plugins_dir.exists() {
            return plugins;
        }
        if let Ok(entries) = std::fs::read_dir(&self.plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    plugins.push(PluginRecord {
                        name,
                        plugin_type: PluginType::Asdf,
                        source: None,
                        path,
                    });
                }
            }
        }
        plugins
    }
}

/// Install a plugin from git.
pub async fn install_git_plugin(
    registry: &PluginRegistry,
    name: &str,
    url: &str,
) -> Result<(), PluginError> {
    let path = registry.plugin_path(name);
    if path.exists() {
        return Err(PluginError::AlreadyInstalled(name.to_string()));
    }
    std::fs::create_dir_all(&path)?;

    let status = tokio::process::Command::new("git")
        .args(["clone", "--depth", "1", url])
        .arg(&path)
        .status()
        .await
        .map_err(|e| PluginError::Git(e.to_string()))?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&path);
        return Err(PluginError::Git(format!("git clone failed for {url}")));
    }
    Ok(())
}

/// Install a plugin from a local directory (copies it).
pub fn install_local_plugin(
    registry: &PluginRegistry,
    name: &str,
    local_path: &Path,
) -> Result<(), PluginError> {
    let dest = registry.plugin_path(name);
    if dest.exists() {
        return Err(PluginError::AlreadyInstalled(name.to_string()));
    }
    std::fs::create_dir_all(&dest)?;
    copy_dir_recursive(local_path, &dest)?;
    Ok(())
}

/// Uninstall a plugin.
pub fn uninstall_plugin(
    registry: &PluginRegistry,
    name: &str,
) -> Result<(), PluginError> {
    let path = registry.plugin_path(name);
    if !path.exists() {
        return Err(PluginError::NotInstalled(name.to_string()));
    }
    std::fs::remove_dir_all(&path)?;
    Ok(())
}

/// Update a plugin (git pull).
pub async fn update_git_plugin(
    registry: &PluginRegistry,
    name: &str,
) -> Result<(), PluginError> {
    let path = registry.plugin_path(name);
    if !path.exists() {
        return Err(PluginError::NotInstalled(name.to_string()));
    }
    let status = tokio::process::Command::new("git")
        .args(["pull", "--ff-only"])
        .current_dir(&path)
        .status()
        .await
        .map_err(|e| PluginError::Git(e.to_string()))?;
    if !status.success() {
        return Err(PluginError::Git(format!("git pull failed for {name}")));
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)?;
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

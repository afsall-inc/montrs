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

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tool entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryTool {
    /// Short name (e.g. "rust", "node").
    #[serde(default)]
    pub name: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: String,
    /// Backend sources in priority order (e.g. ["core:rust", "asdf:code-lever/asdf-rust"]).
    #[serde(default)]
    pub backends: Vec<String>,
    /// Binary names installed by this tool.
    #[serde(default)]
    pub bins: Vec<String>,
    /// File patterns that detect this tool.
    #[serde(default)]
    pub detect: Vec<String>,
    /// Idiomatic version files.
    #[serde(default)]
    pub idiomatic_files: Vec<String>,
    /// Aliases for this tool.
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    /// Platform-specific overrides.
    #[serde(default)]
    pub platform: HashMap<String, PlatformOverride>,
    /// Backend options (e.g. GitHub asset template for raw binaries).
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

impl RegistryTool {
    /// Read a string option by key.
    pub fn option_str(&self, key: &str) -> Option<String> {
        self.options
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Read a boolean option by key.
    pub fn option_bool(&self, key: &str) -> bool {
        self.options
            .get(key)
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}

/// Platform-specific overrides for a tool.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformOverride {
    #[serde(default)]
    pub backends: Vec<String>,
    #[serde(default)]
    pub bins: Vec<String>,
}

/// The full registry — a map of tool names to their definitions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Registry {
    pub tools: HashMap<String, RegistryTool>,
}

impl Registry {
    /// Look up a tool by name.
    pub fn get(&self, name: &str) -> Option<&RegistryTool> {
        self.tools.get(name)
    }

    /// Check if a tool is in the registry.
    pub fn has(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Search tools by name, description, or binary.
    pub fn search(&self, query: &str) -> Vec<&RegistryTool> {
        let q = query.to_lowercase();
        self.tools
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&q)
                    || t.description.to_lowercase().contains(&q)
                    || t.bins.iter().any(|b| b.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// Number of tools in the registry.
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// Get the best backend for a tool on the current platform.
    pub fn best_backend(&self, name: &str) -> Option<&str> {
        let tool = self.tools.get(name)?;
        // Check platform-specific overrides first
        let os = std::env::consts::OS;
        if let Some(platform) = tool.platform.get(os)
            && !platform.backends.is_empty()
        {
            return platform.backends.first().map(|s| s.as_str());
        }
        tool.backends.first().map(|s| s.as_str())
    }
}

/// The baked-in registry, compiled from `registry/*.toml` at build time.
pub static BAKED_REGISTRY: Lazy<Registry> = Lazy::new(|| {
    let mut tools = HashMap::new();
    // Use include_str! to embed registry files at compile time
    let cargo_toml = include_str!("../registry/cargo.toml");
    let rust_toml = include_str!("../registry/rust.toml");
    let forehead_toml = include_str!("../registry/forehead.toml");
    let changelogger_toml = include_str!("../registry/changelogger.toml");
    let tailwindcss_toml = include_str!("../registry/tailwindcss.toml");
    let wasm_bindgen_toml = include_str!("../registry/wasm-bindgen.toml");
    for (name, content) in [
        ("cargo", cargo_toml),
        ("rust", rust_toml),
        ("forehead", forehead_toml),
        ("changelogger", changelogger_toml),
        ("tailwindcss", tailwindcss_toml),
        ("wasm-bindgen", wasm_bindgen_toml),
    ] {
        if let Ok(mut tool) = toml::from_str::<RegistryTool>(content) {
            tool.name = name.to_string();
            tools.insert(name.to_string(), tool);
        }
    }
    Registry { tools }
});

/// Load registry from a directory of TOML files.
pub fn load_registry_from_dir(
    path: &std::path::Path,
) -> Result<Registry, std::io::Error> {
    let mut tools = HashMap::new();
    if !path.exists() {
        return Ok(Registry { tools });
    }
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            let content = std::fs::read_to_string(&path)?;
            if let Ok(mut tool) = toml::from_str::<RegistryTool>(&content) {
                tool.name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                tools.insert(tool.name.clone(), tool);
            }
        }
    }
    Ok(Registry { tools })
}

/// Fetch the floating registry from montrs.com.
pub async fn fetch_floating_registry(
    url: &str,
) -> Result<Registry, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let registry: Registry = response.json().await?;
    Ok(registry)
}

/// Cache path for the floating registry.
pub fn registry_cache_path() -> Option<std::path::PathBuf> {
    dirs::cache_dir().map(|d| d.join("montrs").join("registry.json"))
}

/// Save the floating registry to cache.
pub fn save_registry_cache(registry: &Registry) -> Result<(), std::io::Error> {
    if let Some(path) = registry_cache_path() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string(registry)?;
        std::fs::write(path, json)?;
    }
    Ok(())
}

/// Load the cached floating registry.
pub fn load_cached_registry() -> Result<Registry, std::io::Error> {
    if let Some(path) = registry_cache_path()
        && path.exists()
    {
        let content = std::fs::read_to_string(path)?;
        let registry: Registry = serde_json::from_str(&content)?;
        return Ok(registry);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No cached registry found",
    ))
}

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

//! MontRS dependency management — parses the `[deps]` section of `montrs.toml`
//! and provides freshness checking for lockfiles and outputs.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// A dependency specification from `montrs.toml` `[deps]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepSpec {
    /// The provider prefix (e.g., "cargo", "git-submodule", "npm").
    pub provider: String,
    /// The target (package name, repo URL, etc.).
    pub target: String,
    /// Whether this dep should auto-install before tasks run.
    #[serde(default)]
    pub auto: bool,
    /// Custom options for the provider.
    #[serde(default)]
    pub options: HashMap<String, String>,
}

impl DepSpec {
    /// Parse a key from `[deps]` like "cargo:ripgrep@14.0.0".
    pub fn parse(key: &str, raw_value: Option<toml::Value>) -> Self {
        if let Some((provider, target)) = key.split_once(':') {
            let (target, _version) =
                target.rsplit_once('@').unwrap_or((target, ""));
            let mut spec = Self {
                provider: provider.to_string(),
                target: target.to_string(),
                auto: false,
                options: HashMap::new(),
            };
            if let Some(toml::Value::Table(table)) = raw_value {
                if let Some(auto) = table.get("auto").and_then(|v| v.as_bool())
                {
                    spec.auto = auto;
                }
                for (k, v) in table {
                    if k != "auto"
                        && let Some(s) = v.as_str()
                    {
                        spec.options.insert(k.clone(), s.to_string());
                    }
                }
            }
            spec
        } else {
            DepSpec {
                provider: "custom".to_string(),
                target: key.to_string(),
                auto: false,
                options: HashMap::new(),
            }
        }
    }
}

/// Errors from dependency operations.
#[derive(Debug, thiserror::Error)]
pub enum DepsError {
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),
    #[error("Dependency '{0}' not found in config")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Provider error: {0}")]
    Provider(String),
}

/// Result of a freshness check between sources and outputs.
#[derive(Debug, Clone)]
pub enum Freshness {
    /// Outputs are up-to-date.
    Fresh,
    /// Outputs don't exist.
    OutputsMissing,
    /// Sources changed since last successful run.
    Stale(String),
    /// Force flag was used.
    Forced,
}

/// A single dependency resolved from config.
#[derive(Debug, Clone)]
pub struct ResolvedDep {
    pub spec: DepSpec,
    pub source_paths: Vec<PathBuf>,
    pub output_paths: Vec<PathBuf>,
    pub is_fresh: Freshness,
}

/// The dependency manager — resolves freshness and triggers installs.
pub struct DepsManager {
    pub deps: HashMap<String, DepSpec>,
    pub project_root: PathBuf,
}

impl DepsManager {
    pub fn new(project_root: &Path) -> Self {
        Self {
            deps: HashMap::new(),
            project_root: project_root.to_path_buf(),
        }
    }

    /// Load deps from `montrs.toml` `[deps]` section.
    pub fn load_from_config(&mut self, raw: &HashMap<String, toml::Value>) {
        for (key, value) in raw {
            let spec = DepSpec::parse(key, Some(value.clone()));
            self.deps.insert(key.clone(), spec);
        }
    }

    /// List all deps.
    pub fn list(&self) -> Vec<&DepSpec> {
        self.deps.values().collect()
    }

    /// Check freshness for a dep.
    pub fn check_freshness(
        &self,
        name: &str,
    ) -> Result<ResolvedDep, DepsError> {
        let spec = self
            .deps
            .get(name)
            .ok_or_else(|| DepsError::NotFound(name.to_string()))?;
        let (source_paths, output_paths) = self.resolve_paths(spec);
        let freshness = self.compute_freshness(&source_paths, &output_paths);
        Ok(ResolvedDep {
            spec: spec.clone(),
            source_paths,
            output_paths,
            is_fresh: freshness,
        })
    }

    /// Compute a hash over source files.
    pub fn compute_source_hash(paths: &[PathBuf]) -> String {
        let mut hasher = Sha256::new();
        for path in paths {
            if path.exists() {
                if path.is_file() {
                    if let Ok(content) = std::fs::read(path) {
                        hasher.update(&content);
                    }
                } else {
                    for entry in walkdir::WalkDir::new(path) {
                        if let Ok(entry) = entry
                            && entry.file_type().is_file()
                            && let Ok(content) = std::fs::read(entry.path())
                        {
                            hasher.update(&content);
                        }
                    }
                }
            }
        }
        hex::encode(hasher.finalize())
    }

    fn resolve_paths(&self, spec: &DepSpec) -> (Vec<PathBuf>, Vec<PathBuf>) {
        match spec.provider.as_str() {
            "cargo" => {
                let sources = vec![
                    self.project_root.join("Cargo.toml"),
                    self.project_root.join("Cargo.lock"),
                ];
                let outputs = vec![self.project_root.join("target")];
                (sources, outputs)
            }
            "git-submodule" => {
                let sources = vec![self.project_root.join(".gitmodules")];
                let outputs =
                    vec![self.project_root.join(".git").join("modules")];
                (sources, outputs)
            }
            "npm" => {
                let sources = vec![
                    self.project_root.join("package.json"),
                    self.project_root.join("package-lock.json"),
                ];
                let outputs = vec![self.project_root.join("node_modules")];
                (sources, outputs)
            }
            _ => (vec![], vec![]),
        }
    }

    fn compute_freshness(
        &self,
        sources: &[PathBuf],
        outputs: &[PathBuf],
    ) -> Freshness {
        // Check if any output is missing.
        if outputs.is_empty() {
            return Freshness::Fresh;
        }
        for output in outputs {
            if !output.exists() {
                return Freshness::OutputsMissing;
            }
        }
        // Check source hash against stored hash.
        let state_dir = self.project_root.join(".montrs/deps");
        let state_file = state_dir.join("state.json");
        if state_file.exists()
            && let Ok(content) = std::fs::read_to_string(&state_file)
        {
            let parsed: serde_json::Result<HashMap<String, String>> =
                serde_json::from_str(&content);
            if let Ok(state) = parsed {
                let current_hash = Self::compute_source_hash(sources);
                if let Some(stored_hash) = state.get("source_hash")
                    && &current_hash != stored_hash
                {
                    return Freshness::Stale(format!(
                        "sources changed (hash: {current_hash} != \
                         {stored_hash})"
                    ));
                }
            }
        }
        Freshness::Fresh
    }

    /// Save the current source hash to the state file.
    pub fn save_state(&self, sources: &[PathBuf]) -> Result<(), DepsError> {
        let state_dir = self.project_root.join(".montrs/deps");
        std::fs::create_dir_all(&state_dir)?;
        let mut state = HashMap::new();
        state.insert(
            "source_hash".to_string(),
            Self::compute_source_hash(sources),
        );
        let json = serde_json::to_string(&state)?;
        std::fs::write(state_dir.join("state.json"), json)?;
        Ok(())
    }
}

/// Common lockfile names to check for freshness.
pub fn known_lockfiles() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Cargo.lock", "target"),
        ("package-lock.json", "node_modules"),
        ("yarn.lock", "node_modules"),
        ("Gemfile.lock", "vendor/bundle"),
        ("go.sum", "vendor"),
        ("poetry.lock", ".venv"),
        ("composer.lock", "vendor"),
    ]
}

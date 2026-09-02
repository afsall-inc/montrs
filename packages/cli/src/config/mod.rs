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

//! Configuration plate for MontRS.

//! This plate defines the structure of the `montrs.toml` configuration file
//! and handles loading/parsing logic. It serves as the central source of truth
//! for project settings, build options, and server configuration.

use anyhow::{Context, Result};
use montrs_fmt::FormatterSettings;
use montrs_metadata::MontrsMetadata;
use serde::{Deserialize, Serialize};

/// The root configuration structure for a MontRS project.
///
/// Corresponds to the `montrs.toml` file. Delegates shared fields to
/// `MontrsMetadata` (the single source of truth), keeping only CLI-specific
/// configuration here.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MontrsConfig {
    /// Core metadata — single source of truth for project identity,
    /// serve, build, deploy, env, tasks, tools, etc.
    #[serde(flatten)]
    pub meta: MontrsMetadata,

    /// E2E testing configuration.
    #[serde(default)]
    pub e2e: E2eConfig,

    /// Formatting configuration.
    #[serde(default)]
    pub fmt: FormatterSettings,

    // Internal CLI fields (not serialized to montrs.toml)
    #[serde(skip)]
    pub verbose: u8,
    #[serde(skip)]
    pub log: Vec<String>,
    #[serde(skip)]
    pub release: bool,
    #[serde(skip)]
    pub hot_reload: bool,
    #[serde(skip)]
    pub features: Vec<String>,
}

/// E2E testing configuration.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct E2eConfig {
    /// Run browsers in headless mode.
    #[serde(default)]
    pub headless: Option<bool>,
    /// Browser to use (chromium, firefox, webkit).
    #[serde(default)]
    pub browser: Option<String>,
    /// Base URL for tests (overrides automatic detection).
    #[serde(default)]
    pub base_url: Option<String>,
}

/// Read the root package name from `Cargo.toml` in the current directory.
/// Avoids invoking `cargo metadata` (a subprocess) just to auto-detect the
/// project name.
fn detect_package_name() -> Option<String> {
    let content = std::fs::read_to_string("Cargo.toml").ok()?;
    let value: toml::Value = toml::from_str(&content).ok()?;
    value
        .get("package")?
        .get("name")?
        .as_str()
        .map(|s| s.to_string())
}

/// Whether the current invocation requested a release (optimized) build.
///
/// The CLI `--release` flag is propagated here via an env var because each
/// subcommand loads its own fresh `MontrsConfig` and the flag lives only on the
/// parsed CLI args.
pub fn current_release() -> bool {
    std::env::var("MONTRS_RELEASE")
        .map(|v| v == "1")
        .unwrap_or(false)
}

impl MontrsConfig {
    /// Loads configuration from a specific file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let content =
            std::fs::read_to_string(path.as_ref()).with_context(|| {
                format!(
                    "Failed to read config file: {}",
                    path.as_ref().display()
                )
            })?;
        let mut config: Self = toml::from_str(&content).with_context(|| {
            format!("Failed to parse config file: {}", path.as_ref().display())
        })?;

        // Auto-detect project name from Cargo.toml if not set
        if config.meta.project.name.is_none()
            && let Some(name) = detect_package_name()
        {
            config.meta.project.name = Some(name);
        }

        Ok(config)
    }

    /// Loads configuration from `montrs.toml` in the current directory.
    ///
    /// If the file is missing, returns default configuration.
    /// Also attempts to resolve the project name from `Cargo.toml`.
    pub fn load() -> Result<Self> {
        let mut config = if std::path::Path::new("montrs.toml").exists() {
            Self::from_file("montrs.toml")?
        } else {
            Self::default()
        };

        // Cascade of Truth: Load montrs-fmt.toml if it exists and override the [fmt] section
        if std::path::Path::new("montrs-fmt.toml").exists() {
            let content = std::fs::read_to_string("montrs-fmt.toml")?;
            if let Ok(fmt_settings) = toml::from_str(&content) {
                config.fmt = fmt_settings;
            }
        }

        // Try to resolve project name if still default
        if config.meta.project.name.is_none()
            && let Some(name) = detect_package_name()
        {
            config.meta.project.name = Some(name);
        }

        Ok(config)
    }
}

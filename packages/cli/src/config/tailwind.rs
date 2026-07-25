// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TailwindToml {
    pub content: Option<Vec<String>>,
    pub theme: Option<serde_json::Value>,
    pub plugins: Option<Vec<String>>,
    pub merge: Option<MergeConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MergeConfig {
    pub prefix: Option<String>,
    pub separator: Option<String>,
}

impl TailwindToml {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content =
            fs::read_to_string(path).context("Failed to read tailwind.toml")?;
        let config: Self = toml::from_str(&content)
            .context("Failed to parse tailwind.toml")?;
        Ok(config)
    }

    pub fn to_js(&self) -> String {
        let content = self
            .content
            .as_ref()
            .map(|c| serde_json::to_string(c).unwrap())
            .unwrap_or_else(|| "[]".to_string());
        let theme = self
            .theme
            .as_ref()
            .map(|t| serde_json::to_string_pretty(t).unwrap())
            .unwrap_or_else(|| "{}".to_string());

        let mut plugins_js = String::new();
        if let Some(plugins) = &self.plugins {
            for plugin in plugins {
                plugins_js.push_str(&format!("require('{}'),", plugin));
            }
        }

        format!(
            "module.exports = {{\n  content: {},\n  theme: {},\n  plugins: \
             [{}]\n}}",
            content, theme, plugins_js
        )
    }
}

pub fn ensure_tailwind_config(
    project_root: &Path,
    style: super::TailwindStyle,
) -> Result<Option<std::path::PathBuf>> {
    if matches!(style, super::TailwindStyle::V4) {
        return Ok(None);
    }

    let toml_path = project_root.join("tailwind.toml");

    // If Toml style is forced, or Auto and toml exists
    if matches!(style, super::TailwindStyle::Toml)
        || (matches!(style, super::TailwindStyle::Auto) && toml_path.exists())
    {
        if !toml_path.exists() && matches!(style, super::TailwindStyle::Toml) {
            bail!("tailwind.toml not found but --tailwind-toml was specified");
        }

        let config = TailwindToml::load(&toml_path)?;
        let js_content = config.to_js();
        let js_path = project_root.join("tailwind.config.js");

        fs::write(&js_path, js_content)
            .context("Failed to write tailwind.config.js")?;
        return Ok(Some(js_path));
    }
    Ok(None)
}

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

use serde::{Deserialize, Serialize};

/// Configuration for the MontRS UI theming system.
/// Mirrors shadcn's components.json structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub style: String,
    pub tailwind: TailwindConfig,
    pub aliases: AliasesConfig,
    #[serde(default = "default_icon_library")]
    pub icon_library: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailwindConfig {
    pub css: String,
    pub toml: Option<String>,
    pub base_color: String,
    #[serde(default = "default_css_variables")]
    pub css_variables: bool,
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasesConfig {
    pub components: String,
    pub utils: String,
    pub ui: Option<String>,
    pub hooks: Option<String>,
}

fn default_icon_library() -> String {
    "montrs".to_string()
}

fn default_css_variables() -> bool {
    true
}

impl ComponentsConfig {
    pub fn default_neutral() -> Self {
        Self {
            schema: Some("https://ui.montrs.com/schema.json".into()),
            style: "default".into(),
            tailwind: TailwindConfig {
                css: "style/main.css".into(),
                toml: Some("tailwind.toml".into()),
                base_color: "neutral".into(),
                css_variables: true,
                prefix: String::new(),
            },
            aliases: AliasesConfig {
                components: "@/components".into(),
                utils: "@/lib/utils".into(),
                ui: Some("@/components/ui".into()),
                hooks: Some("@/hooks".into()),
            },
            icon_library: "montrs".into(),
        }
    }

    pub fn load() -> Option<Self> {
        let path = std::path::Path::new("components.json");
        if !path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("components.json", json)?;
        Ok(())
    }
}

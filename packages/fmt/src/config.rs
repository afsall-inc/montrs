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

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatterSettings {
    pub max_width: usize,
    pub tab_spaces: usize,
    pub indentation_style: IndentationStyle,
    pub newline_style: NewlineStyle,
    pub view: ViewSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndentationStyle {
    Tabs,
    Spaces,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NewlineStyle {
    Unix,
    Windows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewSettings {
    pub closing_tag_style: ClosingTagStyle,
    pub attr_value_brace_style: AttrValueBraceStyle,
    pub macro_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClosingTagStyle {
    Preserve,
    SelfClosing,
    NonSelfClosing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttrValueBraceStyle {
    Always,
    WhenRequired,
    Never,
}

impl Default for FormatterSettings {
    fn default() -> Self {
        Self {
            max_width: 100,
            tab_spaces: 4,
            indentation_style: IndentationStyle::Spaces,
            newline_style: NewlineStyle::Unix,
            view: ViewSettings::default(),
        }
    }
}

impl FormatterSettings {
    /// Load settings using the "Cascade of Truth":
    /// 1. montrs-fmt.toml (if it exists)
    /// 2. [fmt] section in montrs.toml (if it exists)
    /// 3. Default settings
    pub fn load() -> Self {
        // Try montrs-fmt.toml first
        if let Ok(content) = std::fs::read_to_string("montrs-fmt.toml")
            && let Ok(settings) = toml::from_str(&content)
        {
            return settings;
        }

        // Try [fmt] section in montrs.toml
        if let Ok(content) = std::fs::read_to_string("montrs.toml")
            && let Ok(value) = toml::from_str::<toml::Value>(&content)
            && let Some(fmt_value) = value.get("fmt")
            && let Ok(settings) = fmt_value.clone().try_into()
        {
            return settings;
        }

        Self::default()
    }
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            closing_tag_style: ClosingTagStyle::SelfClosing,
            attr_value_brace_style: AttrValueBraceStyle::WhenRequired,
            macro_names: vec!["view".to_string()],
        }
    }
}

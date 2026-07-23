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

use crate::{AgentErrorMetadata, ProjectError};
use regex::Regex;
use std::{fs, path::Path, sync::OnceLock};

static ERROR_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse_rustc_errors(output: &str) -> Vec<ProjectError> {
    let re = ERROR_REGEX.get_or_init(|| {
        Regex::new(r"error\[(?P<code>E\d+)\]: (?P<msg>.*)\n\s+--> (?P<file>.*):(?P<line>\d+):(?P<col>\d+)").unwrap()
    });

    let mut errors = Vec::new();
    for cap in re.captures_iter(output) {
        let code = cap
            .name("code")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let message = cap
            .name("msg")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let file = cap
            .name("file")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let line = cap
            .name("line")
            .and_then(|m| m.as_str().parse::<usize>().ok())
            .unwrap_or(0);
        let column = cap
            .name("col")
            .and_then(|m| m.as_str().parse::<usize>().ok())
            .unwrap_or(0);

        let mut docs = vec![format!(
            "https://doc.rust-lang.org/error-index.html#{}",
            code
        )];

        // Map common errors to MontRS framework invariants if applicable
        match code.as_str() {
            "E0433" | "E0432" => {
                // Missing import/crate - often related to missing dependencies in Cargo.toml
                docs.push("docs/architecture/packages.md".to_string());
            }
            "E0277" | "E0599" => {
                // Trait bounds not met - often related to missing Plate or Route implementation
                docs.push("docs/core/plates.md".to_string());
                docs.push("docs/core/router.md".to_string());
            }
            _ => {}
        }

        // Read code context if the file exists
        let code_context = if line > 0 && Path::new(&file).exists() {
            match fs::read_to_string(&file) {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().collect();
                    let start = line.saturating_sub(3);
                    let end = (line + 2).min(lines.len());

                    let mut context = String::new();
                    for (i, ln) in
                        lines.iter().enumerate().take(end).skip(start)
                    {
                        let line_num = i + 1;
                        let indicator =
                            if line_num == line { "> " } else { "  " };
                        context.push_str(&format!(
                            "{}{:4} | {}\n",
                            indicator, line_num, ln
                        ));
                    }
                    context
                }
                Err(_) => "".to_string(),
            }
        } else {
            "".to_string()
        };

        errors.push(ProjectError {
            package: None, // Will be filled by AgentManager if possible
            file,
            line: line as u32,
            column: column as u32,
            message: message.clone(),
            code_context,
            level: "Error".to_string(),
            agent_metadata: Some(AgentErrorMetadata {
                error_code: code.clone(),
                explanation: format!(
                    "Rust compiler error {}: {}",
                    code, message
                ),
                suggested_fixes: Vec::new(),
                rustc_error: Some(output.to_string()),
                documentation_refs: docs,
            }),
        });
    }
    errors
}


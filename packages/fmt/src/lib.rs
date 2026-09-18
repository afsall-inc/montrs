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

use montrs_core::AgentError;
use std::path::Path;
use thiserror::Error;

pub mod comments;
pub mod config;
pub mod macro_fmt;

pub use config::FormatterSettings;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Parse error: {0}")]
    Parse(#[from] syn::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Macro format error: {0}")]
    Macro(String),
}

impl AgentError for FormatError {
    fn error_code(&self) -> &'static str {
        match self {
            FormatError::Parse(_) => "FMT_PARSE",
            FormatError::Io(_) => "FMT_IO",
            FormatError::Macro(_) => "FMT_MACRO",
        }
    }

    fn explanation(&self) -> String {
        match self {
            FormatError::Parse(e) => {
                format!("Failed to parse Rust source code: {e}.")
            }
            FormatError::Io(e) => {
                format!("An I/O error occurred during formatting: {e}.")
            }
            FormatError::Macro(e) => format!(
                "An error occurred while formatting a MontRS macro: {e}.",
            ),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            FormatError::Parse(_) => vec![
                "Check the Rust source code for syntax errors.".to_string(),
                "Ensure that all macros are properly closed.".to_string(),
            ],
            FormatError::Io(_) => vec![
                "Verify that the file path is correct and accessible."
                    .to_string(),
                "Check for file system permissions.".to_string(),
            ],
            FormatError::Macro(_) => vec![
                "Check the syntax within the view! or other MontRS macros."
                    .to_string(),
                "Ensure that the macro contents follow the expected MontRS \
                 specification."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "fmt"
    }
}

/// Formats a single Rust file.
pub fn format_file(
    path: impl AsRef<Path>,
    settings: &FormatterSettings,
) -> Result<String, FormatError> {
    let source = std::fs::read_to_string(path)?;
    format_source(&source, settings)
}

/// Formats a Rust source string.
pub fn format_source(
    source: &str,
    settings: &FormatterSettings,
) -> Result<String, FormatError> {
    let use_crlf = match settings.newline_style {
        config::NewlineStyle::Windows => true,
        config::NewlineStyle::Unix => false,
        config::NewlineStyle::Auto => source.contains("\r\n"),
    };
    let source = normalize_scaffold_headers(source);
    syn::parse_file(&source)?;
    let formatted = format_rust(&source, settings)?;
    let file = syn::parse_file(&formatted)?;
    let mut rope = crop::Rope::from(formatted);
    let mut edits = Vec::new();
    macro_fmt::collect_and_format_macros(&file, &rope, settings, &mut edits)?;
    macro_fmt::apply_edits(&mut rope, edits);
    let result = rope.to_string();
    syn::parse_file(&result)?;
    if use_crlf {
        Ok(result.replace('\n', "\r\n"))
    } else {
        Ok(result)
    }
}

fn format_rust(
    source: &str,
    settings: &FormatterSettings,
) -> Result<String, FormatError> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    let config = format!(
        "edition=2024,skip_children=true,max_width={},tab_spaces={},\
         hard_tabs={},newline_style=Unix,format_macro_bodies=false",
        settings.max_width,
        settings.tab_spaces,
        settings.indentation_style == config::IndentationStyle::Tabs
    );
    let mut child = Command::new("rustfmt")
        .args(["--emit", "stdout", "--config", &config])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    let output = std::thread::scope(|scope| {
        let writer = scope.spawn(move || stdin.write_all(source.as_bytes()));
        let output = child.wait_with_output();
        writer.join().unwrap()?;
        output
    })?;
    if !output.status.success() {
        return Err(FormatError::Macro(format!(
            "rustfmt failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| FormatError::Macro(error.to_string()))
}

fn normalize_scaffold_headers(source: &str) -> String {
    let mut lines: Vec<String> =
        source.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() {
        return source.to_string();
    }

    // Standardize "MontRS Plate Sketch" and "MontRS Route Sketch" headers
    for line in lines.iter_mut().take(3) {
        if line.trim_start().starts_with("//")
            && line.contains("MontRS")
            && (line.contains("Sketch") || line.contains("Blueprint"))
            && !line.starts_with("//!")
        {
            *line = format!(
                "//! {}",
                line.trim_start_matches(|c: char| !c.is_alphabetic())
            );
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatterSettings;

    #[test]
    fn test_format_basic_rust() {
        let source = "fn main() { let x = 1; }";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("fn main() {"));
        assert!(formatted.contains("let x = 1;"));
    }

    #[test]
    fn test_format_with_comments() {
        let source = "fn main() {\n    // A line comment\n    let x = 1; /* a \
                      block comment */\n}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("// A line comment"));
        assert!(formatted.contains("/* a block comment */"));
    }

    #[test]
    fn test_format_view_macro() {
        let source = "fn main() {\n    view! { <div \
                      class=\"test\"><span>\"Hello\"</span></div> };\n}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("<div class=\"test\">"));
        assert!(formatted.contains("<span>"));
        assert!(formatted.contains("\"Hello\""));
    }

    #[test]
    fn test_normalize_headers() {
        let source = "// MontRS Plate Sketch: Test\nfn main() {}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("//! MontRS Plate Sketch: Test"));
    }

    #[test]
    fn test_agent_tool_preservation() {
        let source = "// @agent-tool: name=\"test\"\nfn main() {}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("@agent-tool"));
        assert!(
            formatted.find("@agent-tool").unwrap()
                < formatted.find("fn main").unwrap()
        );
    }
}

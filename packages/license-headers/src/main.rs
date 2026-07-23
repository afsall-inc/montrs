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

use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

const HEADER_PATH: &str = "docs/LICENSES/headers/HEADER-MIT-APACHE";

fn repo_root() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.parent().unwrap().parent().unwrap().to_owned()
}

fn is_skip_dir(name: &str) -> bool {
    matches!(name, ".git" | "target" | "node_modules")
}

fn find_content_start_rs(lines: &[&str]) -> usize {
    for (i, line) in lines.iter().enumerate() {
        let s = line.trim();
        if s.starts_with("//!") || s.starts_with("///") {
            return i;
        }
        if s.starts_with("#[") || s.starts_with("#![") {
            return i;
        }
        if s.starts_with("use ")
            || s.starts_with("mod ")
            || s.starts_with("pub ")
            || s.starts_with("fn ")
            || s.starts_with("struct ")
            || s.starts_with("enum ")
            || s.starts_with("type ")
            || s.starts_with("impl ")
            || s.starts_with("trait ")
            || s.starts_with("const ")
            || s.starts_with("static ")
            || s.starts_with("macro_rules!")
        {
            return i;
        }
        if s.starts_with("//") || s.is_empty() {
            continue;
        }
        return i;
    }
    lines.len()
}

fn apply_header_to_rs(content: &str, header: &str) -> Option<String> {
    if content.trim_start().starts_with(header) {
        return None;
    }
    let lines: Vec<&str> = content.lines().collect();
    let start = find_content_start_rs(&lines);
    let mut remaining: Vec<&str> = lines[start..].to_vec();
    while remaining.first().map_or(false, |l| l.trim().is_empty()) {
        remaining.remove(0);
    }
    Some(format!("{header}\n\n{}\n", remaining.join("\n")))
}

fn update_license_in_toml(content: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    for line in content.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with("license =") && !trimmed.starts_with("license.workspace") {
            let indent = &line[..line.len() - line.trim_start().len()];
            let new_line = format!("{indent}license = \"Apache-2.0 OR MIT\"");
            if line != new_line {
                changed = true;
            }
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    if changed { Some(result) } else { None }
}

fn main() {
    let root = repo_root();
    let header_path = root.join(HEADER_PATH);
    let header = fs::read_to_string(&header_path)
        .unwrap_or_else(|e| panic!("Failed to read header from {}: {e}", header_path.display()));
    let header = header.trim().to_string();

    let mut modified = 0;

    for entry in WalkDir::new(&root).into_iter().filter_entry(|e| {
        !is_skip_dir(e.file_name().to_str().unwrap_or(""))
    }) {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        if ext == "rs" {
            let content = fs::read_to_string(path).unwrap();
            if let Some(new) = apply_header_to_rs(&content, &header) {
                fs::write(path, &new).unwrap();
                println!("{}", path.strip_prefix(&root).unwrap().display());
                modified += 1;
            }
        } else if path.file_name().unwrap_or_default() == "Cargo.toml" {
            let content = fs::read_to_string(path).unwrap();
            if let Some(new) = update_license_in_toml(&content) {
                fs::write(path, &new).unwrap();
                println!("{}", path.strip_prefix(&root).unwrap().display());
                modified += 1;
            }
        }
    }

    if modified == 0 {
        println!("All files up to date.");
    } else {
        eprintln!("Modified {modified} file(s).");
    }
}
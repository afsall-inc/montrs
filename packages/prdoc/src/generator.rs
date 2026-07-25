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

use crate::types::{Audience, BumpLevel, CrateChange, DocSection, PrDoc};
use std::collections::HashSet;

pub struct GenerateOptions {
    pub pr_number: u64,
    pub bump: BumpLevel,
    pub audience: Audience,
    pub force: bool,
}

pub fn generate_prdoc(opts: &GenerateOptions) -> Result<PrDoc, String> {
    let pr_info = fetch_pr_info(opts.pr_number)?;
    let diff = get_pr_diff(opts.pr_number)?;
    let modified_crates = extract_modified_crates(&diff)?;

    let description = pr_info.body.unwrap_or_else(|| "...".to_string());

    let doc = vec![DocSection {
        audience: opts.audience.clone(),
        description,
        title: None,
    }];

    let crates = if modified_crates.is_empty() {
        vec![]
    } else {
        modified_crates
            .iter()
            .map(|name| CrateChange {
                name: name.clone(),
                bump: opts.bump.clone(),
                validate: true,
                note: None,
            })
            .collect()
    };

    Ok(PrDoc {
        title: pr_info.title,
        author: Some(pr_info.author),
        pr: Some(opts.pr_number),
        doc,
        crates,
        migrations: None,
        host_functions: None,
    })
}

struct PrInfo {
    title: String,
    body: Option<String>,
    author: String,
}

fn fetch_pr_info(pr_number: u64) -> Result<PrInfo, String> {
    let output = std::process::Command::new("gh")
        .args([
            "pr",
            "view",
            &pr_number.to_string(),
            "--json",
            "title,body,author",
        ])
        .output()
        .map_err(|e| format!("Failed to run gh CLI: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "gh CLI failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let title = json["title"].as_str().unwrap_or("...").to_string();
    let body = json["body"].as_str().map(|s| s.to_string());
    let author = json["author"]["login"]
        .as_str()
        .unwrap_or("@unknown")
        .to_string();

    Ok(PrInfo {
        title,
        body,
        author,
    })
}

fn get_pr_diff(pr_number: u64) -> Result<String, String> {
    let output = std::process::Command::new("gh")
        .args(["pr", "diff", &pr_number.to_string()])
        .output()
        .map_err(|e| format!("Failed to run gh CLI: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "gh CLI failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn extract_modified_crates(diff: &str) -> Result<Vec<String>, String> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .map_err(|e| format!("Failed to parse cargo metadata: {e}"))?;

    let workspace_packages: HashSet<&str> = metadata
        .workspace_packages()
        .iter()
        .filter(|pkg| {
            pkg.publish
                .as_ref()
                .map(|p| p.iter().any(|r| r != "rust-analyzer"))
                .unwrap_or(true)
        })
        .map(|pkg| pkg.name.as_str())
        .collect();

    let changed_paths = extract_changed_files(diff);
    let mut crates = HashSet::new();

    for pkg in &metadata.workspace_packages() {
        let pkg_path = pkg.manifest_path.parent().unwrap_or(&pkg.manifest_path);
        for changed_path in &changed_paths {
            if changed_path.starts_with(pkg_path.as_str()) {
                if workspace_packages.contains(pkg.name.as_str()) {
                    crates.insert(pkg.name.clone());
                }
                break;
            }
        }
    }

    let mut sorted: Vec<String> = crates.into_iter().collect();
    sorted.sort();
    Ok(sorted)
}

fn extract_changed_files(diff: &str) -> Vec<String> {
    let mut files = Vec::new();
    for line in diff.lines() {
        if line.starts_with("diff --git")
            && let Some(path) = line
                .splitn(4, ' ')
                .nth(3)
                .map(|p| p.trim_start_matches("b/"))
        {
            files.push(path.to_string());
        }
    }
    files
}

pub fn render_prdoc(prdoc: &PrDoc) -> String {
    let mut out = String::new();

    out.push_str("# PRDoc: Pull Request Documentation\n");
    out.push_str("# Edit the ... placeholders with meaningful content.\n");
    out.push_str("# See docs/contributor/prdoc.md for schema details.\n\n");

    out.push_str("---\n");
    out.push_str(&format!("title: {}\n", escape_yaml_string(&prdoc.title)));

    if let Some(ref author) = prdoc.author {
        out.push_str(&format!("author: {}\n", author));
    }

    if let Some(pr) = prdoc.pr {
        out.push_str(&format!("pr: {}\n", pr));
    }

    out.push_str("\ndoc:\n");
    for doc_section in &prdoc.doc {
        out.push_str(&format!(
            "  - audience: {}\n",
            doc_section.audience.as_str()
        ));
        out.push_str("    description: |\n");
        for line in doc_section.description.lines() {
            out.push_str(&format!("      {}\n", line));
        }
    }

    out.push_str("\ncrates:\n");
    for crate_change in &prdoc.crates {
        out.push_str(&format!("  - name: {}\n", crate_change.name));
        out.push_str(&format!("    bump: {}\n", crate_change.bump.as_str()));
        if !crate_change.validate {
            out.push_str("    validate: false\n");
        }
        if let Some(ref note) = crate_change.note {
            out.push_str(&format!("    note: {}\n", escape_yaml_string(note)));
        }
    }

    if let Some(ref migrations) = prdoc.migrations
        && (!migrations.db.is_empty() || !migrations.runtime.is_empty())
    {
        out.push_str("\nmigrations:\n");
        if !migrations.db.is_empty() {
            out.push_str("  db:\n");
            for mig in &migrations.db {
                out.push_str(&format!("    - name: {}\n", mig.name));
                out.push_str(&format!(
                    "      description: {}\n",
                    escape_yaml_string(&mig.description)
                ));
            }
        }
        if !migrations.runtime.is_empty() {
            out.push_str("  runtime:\n");
            for mig in &migrations.runtime {
                out.push_str("    - description: |\n");
                for line in mig.description.lines() {
                    out.push_str(&format!("        {}\n", line));
                }
                if let Some(ref reference) = mig.reference {
                    out.push_str(&format!(
                        "        reference: {}\n",
                        reference
                    ));
                }
            }
        }
    }

    if let Some(ref host_functions) = prdoc.host_functions
        && !host_functions.is_empty()
    {
        out.push_str("\nhost_functions:\n");
        for hf in host_functions {
            out.push_str(&format!("  - name: {}\n", hf.name));
            out.push_str(&format!(
                "    description: {}\n",
                escape_yaml_string(&hf.description)
            ));
            if let Some(ref notes) = hf.notes {
                out.push_str(&format!(
                    "    notes: {}\n",
                    escape_yaml_string(notes)
                ));
            }
        }
    }

    out.push_str("---\n");

    out
}

fn escape_yaml_string(s: &str) -> String {
    if s.contains(':')
        || s.contains('\n')
        || s.contains('"')
        || s.contains('#')
        || s.is_empty()
    {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

pub fn default_output_path(pr_number: u64) -> String {
    format!("prdoc/pr_{}.prdoc", pr_number)
}

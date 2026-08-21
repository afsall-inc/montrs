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

//! `montrs install` — one-command prerequisites setup.
//!
//! Installs every external tool MontRS needs to build/serve a project
//! without requiring the user to set up npm, Node, or manual toolchains.
//! All binaries are downloaded as standalone executables or via `cargo
//! install` and stored in MontRS' managed install directory.

use anyhow::{Context, Result, bail};
use console::style;
use montrs_lockfile::{
    LockfileTool, MontrsLock, lockfile_path_for_root, write_lockfile,
};
use montrs_tool::{ToolManager, ToolRequest};
use std::{collections::BTreeMap, path::Path, process::Command};

/// Tools installed when a project does not declare its own `[tools]` table.
const DEFAULT_TOOLS: &[(&str, &str)] =
    &[("tailwindcss", "latest"), ("wasm-bindgen", "latest")];

pub async fn run(
    tool: Option<String>,
    force: bool,
    dry_run: bool,
) -> Result<()> {
    let root = std::env::current_dir()
        .context("Failed to read the current directory")?;
    let manager = ToolManager::new();

    // --- 1. Compute the tool set to install --------------------------------
    let requests: Vec<ToolRequest> = if let Some(spec) = tool {
        vec![ToolRequest::parse(&spec)]
    } else {
        project_tool_requests(&root)?
    };

    // --- 2. Ensure the Rust WASM target ------------------------------------
    ensure_wasm_target(dry_run).await?;

    // --- 3. Install each tool ----------------------------------------------
    let mut installed: Vec<(String, String)> = Vec::new();
    let mut failed = 0usize;
    for req in &requests {
        let label = format!(
            "{}@{}",
            req.name,
            req.version.as_deref().unwrap_or("latest")
        );

        if dry_run {
            println!(
                "  {} would install {}",
                style("→").cyan().bold(),
                style(&label).bold()
            );
            installed.push((req.name.clone(), "latest".to_string()));
            continue;
        }

        match install_tool(&manager, req, force).await {
            Ok(version) => {
                println!(
                    "  {} installed {}",
                    style("✓").green().bold(),
                    style(format!("{}@{}", req.name, version)).bold()
                );
                installed.push((req.name.clone(), version));
            }
            Err(e) => {
                failed += 1;
                println!(
                    "  {} {}: {}",
                    style("✘").red().bold(),
                    style(&label).bold(),
                    e
                );
            }
        }
    }

    // --- 4. Write the resolved versions to the lockfile -------------------
    write_install_lockfile(&root, &installed)?;

    // --- 5. Summary --------------------------------------------------------
    if dry_run {
        println!(
            "\n{} Dry run — nothing was changed.",
            style("ℹ").cyan().bold()
        );
        return Ok(());
    }

    if failed > 0 {
        bail!(
            "{} tool(s) failed to install. See the errors above.",
            failed
        );
    }
    if installed.is_empty() {
        println!(
            "\n{} All requested tools are already installed.",
            style("✓").green().bold()
        );
    } else {
        println!(
            "\n{} {} tool(s) installed. Run `montrs serve` to start.",
            style("✓").green().bold(),
            installed.len()
        );
    }
    Ok(())
}

/// Read the tool versions a project needs from `montrs.toml [tools]`.
/// Falls back to the default toolset when the project has none.
fn project_tool_requests(root: &Path) -> Result<Vec<ToolRequest>> {
    let montrs_toml = root.join("montrs.toml");
    if montrs_toml.exists() {
        let content = std::fs::read_to_string(&montrs_toml)?;
        let doc: toml::Value = content
            .parse()
            .context("Failed to parse montrs.toml [tools]")?;
        if let Some(tools) = doc.get("tools").and_then(|t| t.as_table()) {
            let mut requests = Vec::new();
            for (name, value) in tools {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(t) => t
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("latest")
                        .to_string(),
                    _ => "latest".to_string(),
                };
                requests.push(ToolRequest {
                    name: name.clone(),
                    version: Some(version),
                });
            }
            if !requests.is_empty() {
                return Ok(requests);
            }
        }
    }
    Ok(DEFAULT_TOOLS
        .iter()
        .map(|(name, ver)| ToolRequest {
            name: name.to_string(),
            version: Some(ver.to_string()),
        })
        .collect())
}

/// Install a single tool (skipping already-installed unless `--force`).
async fn install_tool(
    manager: &ToolManager,
    req: &ToolRequest,
    force: bool,
) -> Result<String> {
    if !force {
        match manager.list_installed(&req.name) {
            Ok(versions) if !versions.is_empty() => {
                // Prefer the newest installed version.
                let current = versions.first().cloned().unwrap();
                println!(
                    "  {} {} already installed ({}). Use --force to reinstall.",
                    style("•").dim(),
                    req.name,
                    current
                );
                return Ok(current);
            }
            _ => {}
        }
    }
    let version = manager.install(req).await.map_err(|e| {
        anyhow::anyhow!("failed to install {}: {}", req.name, e)
    })?;

    // Create shims for every binary the tool provides.
    if let Some(tool) = manager.lookup(&req.name) {
        let bins = tool.bins.clone();
        for bin in bins {
            let _ = manager.create_shim(&req.name, &bin, &version.version);
        }
    }
    Ok(version.version)
}

/// Ensure the `wasm32-unknown-unknown` Rust target is installed via rustup.
async fn ensure_wasm_target(dry_run: bool) -> Result<()> {
    let list = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .context("rustup not found on PATH. MontRS requires Rust — install it via https://rustup.rs")?;

    let installed = String::from_utf8_lossy(&list.stdout);
    if installed
        .lines()
        .any(|l| l.trim() == "wasm32-unknown-unknown")
    {
        println!("  {} wasm32-unknown-unknown Rust target", style("•").dim());
        return Ok(());
    }

    if dry_run {
        println!(
            "  {} would run `rustup target add wasm32-unknown-unknown`",
            style("→").cyan().bold()
        );
        return Ok(());
    }

    println!(
        "  {} Adding wasm32-unknown-unknown target...",
        style("Installing").cyan().bold()
    );
    let status = Command::new("rustup")
        .args(["target", "add", "wasm32-unknown-unknown"])
        .status()
        .context("Failed to run rustup")?;
    if !status.success() {
        bail!("rustup failed to add the wasm32-unknown-unknown target");
    }
    Ok(())
}

/// Write the resolved tool versions into `montrs.lock`.
fn write_install_lockfile(
    root: &Path,
    installed: &[(String, String)],
) -> Result<()> {
    let lock_path = lockfile_path_for_root(root);
    let mut lock = if lock_path.exists() {
        montrs_lockfile::read_lockfile(&lock_path)?
    } else {
        MontrsLock::new()
    };

    for (name, version) in installed {
        lock.add_tool(
            name,
            LockfileTool {
                version: version.clone(),
                backend: None,
                options: BTreeMap::new(),
                platforms: BTreeMap::new(),
            },
        );
    }
    lock.config_sources.push("montrs install".to_string());
    write_lockfile(&lock_path, &lock)?;
    Ok(())
}

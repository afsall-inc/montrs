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

use anyhow::{Result, anyhow};
use console::style;
use std::{fs, path::Path, process::Command};

/// Available templates and their descriptions.
const TEMPLATES: &[(&str, &str)] = &[
    ("default", "Single-app SSR web application"),
    (
        "monorepo",
        "Monorepo with apps/ + packages/ (renamed from workspace)",
    ),
    ("todo", "Full-stack todo app with auth + database"),
    ("api", "Backend-only API server with auth"),
    ("desktop", "Native desktop app"),
    ("saas", "Full SaaS: auth, org, admin, API keys, services"),
];

/// List available templates.
pub async fn list() -> Result<()> {
    println!("{}", style("Available templates:").bold());
    for (name, desc) in TEMPLATES {
        println!("  {:<12} {}", style(name).cyan(), style(desc).dim());
    }
    Ok(())
}

pub async fn run(name: String, template: String) -> Result<()> {
    println!(
        "{} Creating new MontRS project: {}",
        style("🚀").bold(),
        style(&name).cyan().bold()
    );

    let cwd = std::env::current_dir()?;
    let template_dir = cwd.join("templates").join(&template);
    let dest_dir = cwd.join(&name);

    if !template_dir.exists() {
        return Err(anyhow!(
            "Template '{}' not found at {}. Available templates: {}",
            template,
            template_dir.display(),
            list_templates(&cwd)?
        ));
    }

    if dest_dir.exists() {
        return Err(anyhow!(
            "Directory '{}' already exists. Remove it first or choose a \
             different name.",
            name
        ));
    }

    println!("  Copying template '{}' → '{}'", template, name);
    copy_dir_recursive(&template_dir, &dest_dir)?;

    // Substitute project name in Cargo.toml and montrs.toml
    substitute_project_name(&dest_dir, &name)?;

    // Initialize git
    println!("  Initializing git repository...");
    let _ = Command::new("git")
        .args(["init"])
        .current_dir(&dest_dir)
        .output();

    println!();
    println!(
        "{} Project {} created at {}",
        style("✨").green().bold(),
        style(&name).cyan().bold(),
        style(dest_dir.display()).underlined()
    );
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  montrs serve");

    Ok(())
}

fn list_templates(cwd: &Path) -> Result<String> {
    let dir = cwd.join("templates");
    if !dir.exists() {
        return Ok("none".to_string());
    }
    let mut names: Vec<String> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    Ok(names.join(", "))
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest = dst.join(entry.file_name());

        if path.file_name().is_some_and(|n| n == ".agent") {
            continue;
        }

        if path.is_dir() {
            copy_dir_recursive(&path, &dest)?;
        } else {
            fs::copy(&path, &dest)?;
        }
    }
    Ok(())
}

fn substitute_project_name(dir: &Path, name: &str) -> Result<()> {
    // Walk all .toml files and replace {{project-name}}
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name().to_str().is_some_and(|n| n.ends_with(".toml"))
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
        let new_content = content
            .replace("{{project-name}}", name)
            .replace("{{crate_name}}", &name.replace('-', "_"));
        if content != new_content {
            fs::write(path, new_content)?;
        }
    }
    Ok(())
}

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

use anyhow::Result;
use montrs_agent::AgentManager;

pub async fn run(include_docs: bool, format: String) -> Result<()> {
    let output = run_to_string(include_docs, format).await?;
    println!("{}", output);
    Ok(())
}

pub async fn run_to_string(
    include_docs: bool,
    format: String,
) -> Result<String> {
    let cwd = std::env::current_dir()?;
    let manager = AgentManager::new(&cwd);

    // Agent: Ensure tools.json is updated when running spec
    if let Err(e) = manager.write_tools_spec() {
        eprintln!("Warning: Failed to update tools spec: {}", e);
    }

    let mut snapshot = manager.generate_snapshot("unknown")?;

    // Try to load basic project info from Cargo.toml
    if let Ok(cargo_toml_content) =
        std::fs::read_to_string(cwd.join("Cargo.toml"))
        && let Ok(value) = cargo_toml_content.parse::<toml::Value>()
    {
        if let Some(package) = value.get("package") {
            snapshot.project_name = package
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
        } else if let Some(workspace) = value.get("workspace")
            && let Some(package) = workspace.get("package")
        {
            snapshot.project_name = package
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("workspace")
                .to_string();
        }
    }

    if include_docs {
        // AgentManager already includes some documentation, but we can add more if needed
    }

    let output = match format.as_str() {
        "yaml" => serde_yaml::to_string(&snapshot)?,
        "txt" => format!("{:#?}", snapshot),
        _ => serde_json::to_string_pretty(&snapshot)?,
    };

    Ok(output)
}

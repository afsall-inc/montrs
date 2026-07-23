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

use crate::config::MontrsConfig;
use montrs_runner::TaskRunner;
use std::process::Command;

pub async fn run(task_name: String) -> anyhow::Result<()> {
    // Check for Mise configuration files
    let mise_configs = [
        "mise.toml",
        "mise.local.toml",
        ".mise.toml",
        ".mise.local.toml",
    ];
    let has_mise_config = mise_configs
        .iter()
        .any(|f| std::path::Path::new(f).exists());

    if has_mise_config {
        // Try to delegate to mise
        let status = Command::new("mise").arg("run").arg(&task_name).status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            Ok(_) => anyhow::bail!("Mise task '{}' failed", task_name),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Mise not installed, fall back to internal runner
                eprintln!(
                    "Mise config found but 'mise' command not found. Falling \
                     back to internal runner..."
                );
            }
            Err(e) => anyhow::bail!("Failed to execute mise: {}", e),
        }
    }

    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(config.tasks);
    runner.run(&task_name).await?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    // Check for Mise configuration files
    let mise_configs = [
        "mise.toml",
        "mise.local.toml",
        ".mise.toml",
        ".mise.local.toml",
    ];
    let has_mise_config = mise_configs
        .iter()
        .any(|f| std::path::Path::new(f).exists());

    if has_mise_config {
        // Try to list tasks from mise
        let status = Command::new("mise").arg("tasks").status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            Ok(_) => {
                // If mise tasks fails, we'll fall back to internal runner's list
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Mise not installed, fall back to internal runner
            }
            Err(e) => anyhow::bail!("Failed to execute mise: {}", e),
        }
    }

    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(config.tasks);
    runner.list()?;
    Ok(())
}


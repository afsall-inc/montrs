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

use montrs_lockfile::{
    LockfileTool, MontrsLock, lockfile_path_for_root, write_lockfile,
};
use std::collections::BTreeMap;

pub async fn run() -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let lock_path = lockfile_path_for_root(&root);

    let mut lock = if lock_path.exists() {
        montrs_lockfile::read_lockfile(&lock_path)?
    } else {
        MontrsLock::new()
    };

    let montrs_toml = root.join("montrs.toml");
    if montrs_toml.exists() {
        let content = std::fs::read_to_string(&montrs_toml)?;
        let doc: toml::Table = content.parse()?;
        // Replace so stale entries don't accumulate across runs.
        lock.config_sources.clear();
        if let Some(tools) = doc.get("tools").and_then(|t| t.as_table()) {
            for (name, value) in tools {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(t) => t
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("latest")
                        .to_string(),
                    _ => continue,
                };
                lock.set_tool(
                    name,
                    LockfileTool {
                        version,
                        backend: None,
                        options: BTreeMap::new(),
                        platforms: BTreeMap::new(),
                    },
                );
            }
        }
    }

    lock.config_sources.push("montrs.toml".to_string());
    write_lockfile(&lock_path, &lock)?;
    println!("Lockfile written to {}", lock_path.display());
    println!("{} tools locked", lock.len());
    Ok(())
}

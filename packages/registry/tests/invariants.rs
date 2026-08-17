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

//! Invariant tests for montrs-registry.

use montrs_registry::*;
use std::collections::HashMap;

#[test]
fn test_baked_registry_has_tools() {
    // The baked registry is compiled from registry/*.toml
    assert!(!BAKED_REGISTRY.is_empty());
    assert!(BAKED_REGISTRY.has("cargo"));
    assert!(BAKED_REGISTRY.has("rust"));
    assert!(BAKED_REGISTRY.has("forehead"));
    assert!(BAKED_REGISTRY.has("changelogger"));
}

#[test]
fn test_baked_registry_lookup() {
    let rust = BAKED_REGISTRY.get("rust").expect("rust tool");
    assert_eq!(rust.name, "rust");
    assert!(!rust.description.is_empty());
    assert!(!rust.bins.is_empty());
    assert!(rust.bins.contains(&"rustc".to_string()));
}

#[test]
fn test_baked_registry_missing_tool() {
    assert!(!BAKED_REGISTRY.has("nonexistent-tool"));
    assert!(BAKED_REGISTRY.get("nonexistent-tool").is_none());
}

#[test]
fn test_registry_search() {
    let results = BAKED_REGISTRY.search("rust");
    assert!(!results.is_empty());
    // At least one result should be the rust tool itself
    assert!(results.iter().any(|t| t.name == "rust"));
}

#[test]
fn test_registry_len() {
    assert!(BAKED_REGISTRY.len() >= 4);
}

#[test]
fn test_best_backend() {
    let backend = BAKED_REGISTRY.best_backend("cargo");
    assert!(backend.is_some());
}

#[test]
fn test_load_registry_from_dir() {
    let dir = tempfile::tempdir().unwrap();
    let registry_dir = dir.path().join("registry");
    std::fs::create_dir_all(&registry_dir).unwrap();
    std::fs::write(
        registry_dir.join("test-tool.toml"),
        "backends = [\"core:test\"]\nbins = [\"test-bin\"]\ndescription = \
         \"Test tool\"\n",
    )
    .unwrap();
    let registry = load_registry_from_dir(&registry_dir).unwrap();
    assert_eq!(registry.len(), 1);
    assert!(registry.has("test-tool"));
    let tool = registry.get("test-tool").unwrap();
    assert_eq!(tool.bins, vec!["test-bin"]);
}

#[test]
fn test_load_registry_empty_dir() {
    let dir = tempfile::tempdir().unwrap();
    let registry = load_registry_from_dir(dir.path()).unwrap();
    assert!(registry.is_empty());
}

#[test]
fn test_registry_tool_serde() {
    let tool = RegistryTool {
        name: "test".to_string(),
        description: "desc".to_string(),
        backends: vec!["core:test".to_string()],
        bins: vec!["t".to_string()],
        detect: vec![],
        idiomatic_files: vec![],
        aliases: HashMap::new(),
        platform: HashMap::new(),
    };
    let json = serde_json::to_string(&tool).unwrap();
    let parsed: RegistryTool = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "test");
    assert_eq!(parsed.backends, vec!["core:test"]);
}

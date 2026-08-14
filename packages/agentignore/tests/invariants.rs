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

//! Invariant tests for montrs-agentignore.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - .agentignore is canonical source of truth
//! - Patterns follow .gitignore syntax
//! - IDE export works correctly

use montrs_agentignore::*;
use std::{fs, path::Path};

fn setup_agentignore(root: &Path) {
    fs::write(root.join(".agentignore"), "target/\n*.rs.bk\n.secrets/\n")
        .expect("failed to write .agentignore");
}

#[test]
fn test_agentignore_load() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    setup_agentignore(root);

    let ai = AgentIgnore::load(root).unwrap();
    assert!(!ai.patterns().is_empty());
    assert!(ai.patterns().contains(&"target/".to_string()));
}

#[test]
fn test_agentignore_load_missing() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let ai = AgentIgnore::load(root).unwrap();
    assert!(ai.patterns().is_empty());
}

#[test]
fn test_agentignore_is_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir_all(root.join("target")).unwrap();
    std::fs::create_dir_all(root.join("src")).unwrap();
    setup_agentignore(root);

    let ai = AgentIgnore::load(root).unwrap();
    assert!(ai.is_ignored(&root.join("target")));
    assert!(!ai.is_ignored(&root.join("src")));
}

#[test]
fn test_agentignore_check_path() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir_all(root.join("target")).unwrap();
    std::fs::create_dir_all(root.join("src")).unwrap();
    setup_agentignore(root);

    assert!(AgentIgnore::check_path(root, "target").unwrap());
    assert!(!AgentIgnore::check_path(root, "src").unwrap());
}

#[test]
fn test_agentignore_export_opencode() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    setup_agentignore(root);

    let result = AgentIgnore::export_for_ide(root, "opencode").unwrap();
    assert!(result.contains("opencodeignore"));
    assert!(root.join(".opencodeignore").exists());
}

#[test]
fn test_agentignore_export_cursor() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    setup_agentignore(root);

    let result = AgentIgnore::export_for_ide(root, "cursor").unwrap();
    assert!(result.contains("cursorignore"));
    assert!(root.join(".cursorignore").exists());
}

#[test]
fn test_agentignore_export_unknown_ide() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    setup_agentignore(root);

    let result = AgentIgnore::export_for_ide(root, "vscode");
    assert!(result.is_err());
}

#[test]
fn test_agentignore_create_from_gitignore() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join(".gitignore"), "node_modules/\n.env\n").unwrap();

    let patterns = AgentIgnore::create_from_gitignore(root).unwrap();
    assert!(patterns.contains(&"target/".to_string()));
    assert!(patterns.contains(&"node_modules/".to_string()));
    assert!(patterns.contains(&".env".to_string()));
}

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

//! Invariant tests for montrs-deps.

use montrs_deps::*;
use std::{collections::HashMap, path::Path};

#[test]
fn test_dep_spec_parse_cargo() {
    let spec = DepSpec::parse("cargo:ripgrep", None);
    assert_eq!(spec.provider, "cargo");
    assert_eq!(spec.target, "ripgrep");
    assert!(!spec.auto);
}

#[test]
fn test_dep_spec_parse_with_version() {
    let spec = DepSpec::parse("cargo:ripgrep@14.0.0", None);
    assert_eq!(spec.provider, "cargo");
    assert_eq!(spec.target, "ripgrep");
}

#[test]
fn test_dep_spec_parse_git() {
    let spec =
        DepSpec::parse("git-submodule:https://github.com/foo/bar.git", None);
    assert_eq!(spec.provider, "git-submodule");
}

#[test]
fn test_dep_spec_parse_no_provider() {
    let spec = DepSpec::parse("my-tool", None);
    assert_eq!(spec.provider, "custom");
    assert_eq!(spec.target, "my-tool");
}

#[test]
fn test_dep_spec_auto() {
    let mut table = toml::map::Map::new();
    table.insert("auto".to_string(), toml::Value::Boolean(true));
    let spec = DepSpec::parse("npm:react", Some(toml::Value::Table(table)));
    assert!(spec.auto);
}

#[test]
fn test_deps_manager_empty() {
    let manager = DepsManager::new(Path::new("/tmp/test"));
    assert!(manager.list().is_empty());
}

#[test]
fn test_deps_manager_load() {
    let mut manager = DepsManager::new(Path::new("/tmp/test"));
    let mut raw = HashMap::new();
    raw.insert(
        "cargo:ripgrep".to_string(),
        toml::Value::String("14.0.0".to_string()),
    );
    manager.load_from_config(&raw);
    assert_eq!(manager.list().len(), 1);
}

#[test]
fn test_check_freshness_not_found() {
    let manager = DepsManager::new(Path::new("/tmp/test"));
    let result = manager.check_freshness("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_deps_manager_load_and_check() {
    let dir = tempfile::tempdir().unwrap();
    let mut raw = HashMap::new();
    raw.insert(
        "cargo:test".to_string(),
        toml::Value::String("1.0".to_string()),
    );
    let mut manager = DepsManager::new(dir.path());
    manager.load_from_config(&raw);
    let list = manager.list();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].provider, "cargo");
}

#[test]
fn test_known_lockfiles() {
    let lockfiles = known_lockfiles();
    assert!(!lockfiles.is_empty());
    assert!(lockfiles.iter().any(|(lf, _)| *lf == "Cargo.lock"));
}

#[test]
fn test_freshness_enum() {
    let fresh = Freshness::Fresh;
    assert!(matches!(fresh, Freshness::Fresh));
    let missing = Freshness::OutputsMissing;
    assert!(matches!(missing, Freshness::OutputsMissing));
    let stale = Freshness::Stale("test".to_string());
    assert!(matches!(stale, Freshness::Stale(_)));
}

#[test]
fn test_deps_error_display() {
    let err = DepsError::UnknownProvider("test".to_string());
    assert!(err.to_string().contains("test"));
    let err = DepsError::NotFound("dep".to_string());
    assert!(err.to_string().contains("dep"));
}

#[test]
fn test_save_and_check_state() {
    let dir = tempfile::tempdir().unwrap();
    let sources = vec![dir.path().join("test.json")];
    std::fs::write(&sources[0], b"hello").unwrap();
    let manager = DepsManager::new(dir.path());
    manager.save_state(&sources).unwrap();
    let state_dir = dir.path().join(".montrs").join("deps");
    assert!(state_dir.join("state.json").exists());
}

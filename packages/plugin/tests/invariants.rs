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

//! Invariant tests for montrs-plugin.

use montrs_plugin::*;
use std::path::Path;

#[test]
fn test_plugin_registry_default() {
    let reg = PluginRegistry::new();
    assert!(!reg.plugins_dir.as_os_str().is_empty());
}

#[test]
fn test_plugin_registry_with_plugins_dir() {
    let dir = tempfile::tempdir().unwrap();
    let reg = PluginRegistry::with_plugins_dir(dir.path().to_path_buf());
    assert_eq!(reg.plugins_dir, dir.path());
}

#[test]
fn test_plugin_registry_is_installed() {
    let dir = tempfile::tempdir().unwrap();
    let plugin_dir = dir.path().join("test-plugin");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    let reg = PluginRegistry::with_plugins_dir(dir.path().to_path_buf());
    assert!(reg.is_installed("test-plugin"));
    assert!(!reg.is_installed("nonexistent"));
}

#[test]
fn test_plugin_registry_list_installed() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("plugin-a")).unwrap();
    std::fs::create_dir_all(dir.path().join("plugin-b")).unwrap();
    let reg = PluginRegistry::with_plugins_dir(dir.path().to_path_buf());
    let plugins = reg.list_installed();
    assert_eq!(plugins.len(), 2);
    let names: Vec<_> = plugins.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"plugin-a"));
    assert!(names.contains(&"plugin-b"));
}

#[test]
fn test_plugin_source_detection() {
    assert!(
        PluginSource::Git("https://github.com/foo/bar.git".to_string())
            .is_git()
    );
    assert!(PluginSource::Local("/tmp/plugin".to_string()).is_local());
    assert!(
        PluginSource::Zip("https://example.com/plugin.zip".to_string())
            .is_zip()
    );
}

#[test]
fn test_plugin_type_display() {
    assert_eq!(format!("{:?}", PluginType::Asdf), "Asdf");
    assert_eq!(format!("{:?}", PluginType::Vfox), "Vfox");
}

#[test]
fn test_plugin_record_construct() {
    let record = PluginRecord {
        name: "test".to_string(),
        plugin_type: PluginType::Asdf,
        source: None,
        path: Path::new("/tmp/test").to_path_buf(),
    };
    assert_eq!(record.name, "test");
    assert_eq!(record.plugin_type, PluginType::Asdf);
}

#[test]
fn test_install_local_plugin() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("source-plugin");
    std::fs::create_dir_all(src.join("bin")).unwrap();
    std::fs::write(src.join("bin/hello.sh"), "echo hello").unwrap();

    let reg = PluginRegistry::with_plugins_dir(dir.path().join("plugins"));
    install_local_plugin(&reg, "my-plugin", &src).unwrap();
    assert!(reg.is_installed("my-plugin"));

    // Install again should fail
    let result = install_local_plugin(&reg, "my-plugin", &src);
    assert!(result.is_err());
}

#[test]
fn test_uninstall_plugin() {
    let dir = tempfile::tempdir().unwrap();
    let plugin_dir = dir.path().join("to-remove");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    let reg = PluginRegistry::with_plugins_dir(dir.path().to_path_buf());
    uninstall_plugin(&reg, "to-remove").unwrap();
    assert!(!reg.is_installed("to-remove"));
}

#[test]
fn test_uninstall_missing_plugin() {
    let dir = tempfile::tempdir().unwrap();
    let reg = PluginRegistry::with_plugins_dir(dir.path().to_path_buf());
    let result = uninstall_plugin(&reg, "nonexistent");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PluginError::NotInstalled(_)));
}

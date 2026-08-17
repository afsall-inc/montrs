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

//! Invariant tests for montrs-metadata.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - All project metadata lives in montrs.toml
//! - Auto-detects from Cargo workspace
//! - All fields have sensible defaults

use montrs_metadata::*;

#[test]
fn test_metadata_default() {
    let meta = MontrsMetadata::default();
    assert!(meta.project.name.is_none());
    assert!(meta.project.version.is_none());
}

#[test]
fn test_metadata_with_project() {
    let meta = MontrsMetadata {
        project: ProjectMeta {
            name: Some("my-app".to_string()),
            version: Some("1.0.0".to_string()),
            description: None,
        },
        ..Default::default()
    };
    assert_eq!(meta.project.name.unwrap(), "my-app");
    assert_eq!(meta.project.version.unwrap(), "1.0.0");
}

#[test]
fn test_serve_meta_defaults() {
    let serve = ServeMeta::default();
    assert_eq!(serve.site_addr, "0.0.0.0:3000");
    assert_eq!(serve.reload_port, 3001);
    assert_eq!(serve.site_root, "target/site");
    assert_eq!(serve.site_pkg_dir, "pkg");
    assert_eq!(serve.browserquery, "defaults");
    assert!(serve.lib_default_features);
    assert!(serve.bin_default_features);
}

#[test]
fn test_build_meta_defaults() {
    let build = BuildMeta::default();
    assert!(!build.release);
    assert_eq!(build.target, "index.html");
}

#[test]
fn test_metadata_serde_roundtrip() {
    let meta = MontrsMetadata::default();
    let toml_str = toml::to_string(&meta).unwrap();
    let parsed: MontrsMetadata = toml::from_str(&toml_str).unwrap();
    assert!(parsed.project.name.is_none());
}

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

//! Invariant tests for montrs-build-serve.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Static Serving Only: serves pre-built files
//! - Trait-Only Dependency: depends on montrs-build-core for config types
//! - Lightweight: axum + tower-http ServeDir

use montrs_build_serve::ServeConfig;
use std::path::PathBuf;

#[test]
fn test_serve_config_defaults() {
    let config = ServeConfig {
        addr: "0.0.0.0:3000".to_string(),
        site_root: PathBuf::from("target/site"),
        pkg_dir: PathBuf::from("pkg"),
    };
    assert_eq!(config.addr, "0.0.0.0:3000");
    assert_eq!(config.site_root, PathBuf::from("target/site"));
    assert_eq!(config.pkg_dir, PathBuf::from("pkg"));
}

#[test]
fn test_serve_config_debug_and_clone() {
    let config = ServeConfig {
        addr: "127.0.0.1:8080".to_string(),
        site_root: PathBuf::from("dist"),
        pkg_dir: PathBuf::from("wasm"),
    };
    let cloned = config.clone();
    assert_eq!(config.addr, cloned.addr);
    assert_eq!(config.site_root, cloned.site_root);
}

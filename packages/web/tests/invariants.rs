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

//! Invariant tests for montrs-web.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - PlatformAdapter Implementation
//! - WASM-First: primary target wasm32-unknown-unknown
//! - No Leptos Dependency

use montrs_platform::{PlatformAdapter, Target};
use montrs_web::*;

#[test]
fn test_web_adapter_construct() {
    let adapter = WebAdapter::new();
    assert_eq!(adapter.target(), Target::Web);
}

#[test]
fn test_web_adapter_default() {
    let adapter = WebAdapter::default();
    assert_eq!(adapter.target(), Target::Web);
}

#[test]
fn test_web_adapter_with_target() {
    let adapter = WebAdapter::with_target(Target::Web);
    assert_eq!(adapter.target(), Target::Web);
}

#[test]
#[should_panic(expected = "WebAdapter requires a web target")]
fn test_web_adapter_rejects_non_web() {
    let _adapter = WebAdapter::with_target(Target::Desktop);
}

#[test]
fn test_web_adapter_description() {
    let adapter = WebAdapter::new();
    assert!(!adapter.description().is_empty());
}

#[test]
fn test_web_adapter_noop_non_wasm() {
    let adapter = WebAdapter::new();
    adapter.open_url("https://example.com");
    adapter.set_title("test");
    adapter.set_size(1024, 768);
}

#[test]
fn test_web_adapter_platform_adapter_trait() {
    let adapter: Box<dyn PlatformAdapter> = Box::new(WebAdapter::new());
    assert_eq!(adapter.target(), Target::Web);
}

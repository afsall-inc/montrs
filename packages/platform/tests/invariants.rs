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

//! Invariant tests for montrs-platform.
//!
//! Validates the invariants defined in `docs/invariants.md`.

use montrs_platform::*;

#[test]
fn test_target_enum_values() {
    assert!(Target::Web.is_web());
    assert!(Target::Desktop.is_desktop());
    assert!(Target::Mobile.is_mobile());
    assert!(Target::Tui.is_tui());
}

#[test]
fn test_target_description_not_empty() {
    for target in &[Target::Web, Target::Desktop, Target::Mobile, Target::Tui] {
        assert!(!target.description().is_empty());
    }
}

#[test]
fn test_noop_platform_adapter() {
    let adapter = NoopPlatformAdapter::new(Target::Web);
    assert_eq!(adapter.target(), Target::Web);
    assert!(!adapter.description().is_empty());
    // No-op methods should not panic
    adapter.open_url("https://example.com");
    adapter.set_title("test");
    adapter.set_size(800, 600);
}

#[test]
fn test_platform_adapter_trait_is_object_safe() {
    // Verify the trait can be used as a trait object
    let adapter: Box<dyn PlatformAdapter> =
        Box::new(NoopPlatformAdapter::new(Target::Web));
    assert_eq!(adapter.target(), Target::Web);
}

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

//! Invariant tests for montrs-utils.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Side-Effect Free: pure functions
//! - Generic Utility: truly generic logic
//! - High Stability: low-level dependency

use montrs_utils::*;

#[test]
fn test_to_pascal_case() {
    assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
    assert_eq!(to_pascal_case("hello world"), "HelloWorld");
    assert_eq!(to_pascal_case("HelloWorld"), "HelloWorld");
    assert_eq!(to_pascal_case(""), "");
}

#[test]
fn test_to_snake_case() {
    assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(to_snake_case("hello-world"), "hello_world");
    assert_eq!(to_snake_case("hello world"), "hello_world");
    assert_eq!(to_snake_case("hello"), "hello");
    assert_eq!(to_snake_case(""), "");
}

#[test]
fn test_to_kebab_case() {
    assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
    assert_eq!(to_kebab_case("hello_world"), "hello-world");
    assert_eq!(to_kebab_case("hello world"), "hello-world");
    assert_eq!(to_kebab_case("hello"), "hello");
}

#[test]
fn test_conversion_roundtrip() {
    let original = "my_variable_name";
    let pascal = to_pascal_case(original);
    let snake = to_snake_case(&pascal);
    assert_eq!(snake, original);
}

#[test]
fn test_no_side_effects() {
    let input = "test_input".to_string();
    let _result = to_pascal_case(&input);
    assert_eq!(input, "test_input");
}

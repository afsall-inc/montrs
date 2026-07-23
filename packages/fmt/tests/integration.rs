// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use montrs_fmt::{FormatterSettings, format_source};

#[test]
fn test_integration_full_formatting() {
    let source = r#"
fn main() {
    // Top level comment
    let x = 1;
    view! {
        <div class="container">
            // Nested comment
            <span>"Hello MontRS"</span>
        </div>
    };
}
"#;

    let settings = FormatterSettings::default();
    let result = format_source(source, &settings).expect("Formatting failed");

    // Check for preservation of essential elements
    assert!(result.contains("fn main()"), "Function signature lost");
    assert!(
        result.contains("// Top level comment"),
        "Top level comment lost"
    );
    assert!(result.contains("// Nested comment"), "Nested comment lost");
    assert!(
        result.contains(r#"class="container""#),
        "Macro attribute lost"
    );
    assert!(result.contains("Hello MontRS"), "Macro text content lost");
}

#[test]
fn test_integration_no_macros() {
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
    let settings = FormatterSettings::default();
    let result = format_source(source, &settings).expect("Formatting failed");

    assert!(result.contains("fn add(a: i32, b: i32) -> i32 {"));
    assert!(result.contains("a + b"));
}

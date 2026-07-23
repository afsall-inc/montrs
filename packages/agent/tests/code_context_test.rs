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

use montrs_agent::error_parser::parse_rustc_errors;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_code_context_reading() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("main.rs");

    let content = r#"fn main() {
    let x = 5;
    println!("{}", x)
    let y = 10;
}
"#;
    fs::write(&file_path, content).unwrap();

    // Mock rustc output pointing to the temp file
    let rustc_output = format!(
        "error[E0425]: cannot find value `z` in this scope\n  --> {}:3:5",
        file_path.to_str().unwrap()
    );

    let errors = parse_rustc_errors(&rustc_output);

    assert_eq!(errors.len(), 1);
    let error = &errors[0];

    // Check that code context was read and includes the line number and marker
    assert!(
        error
            .code_context
            .contains(">    3 |     println!(\"{}\", x)")
    );
    assert!(error.code_context.contains("     2 |     let x = 5;"));
    assert!(error.code_context.contains("     4 |     let y = 10;"));
}

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

fn assert_stable(source: &str) -> String {
    let settings = FormatterSettings::default();
    let result = format_source(source, &settings).unwrap();
    syn::parse_file(&result).unwrap();
    assert_eq!(result, format_source(&result, &settings).unwrap());
    result
}

#[test]
fn preserves_comment_placement_and_duplicates() {
    let source = "fn main() {\n    let first=1; // repeated\n    // \
                  @agent-tool: second\n    let second=2; // repeated\n    let \
                  third=/* outer /* inner */ end */3;\n}\n";
    let result = assert_stable(source);
    assert_eq!(result.matches("// repeated").count(), 2);
    assert!(result.contains("let first = 1; // repeated"));
    assert!(result.contains("// @agent-tool: second\n    let second"));
    assert!(result.contains("/* outer /* inner */ end */"));
}

#[test]
fn preserves_literals_and_header_like_strings() {
    let source = "fn main() {\n    let title = \"MontRS Plate Sketch\";\n    let url = \"https://example.test/path\";\n    let raw = r###\"a \\\" // text /* text */\"###;\n    let byte = b'/';\n    let character = '\"';\n}\n";
    let result = assert_stable(source);
    assert!(result.contains("let title"));
    assert!(result.contains("https://example.test/path"));
    assert!(montrs_fmt::comments::extract_comments(source).1.is_empty());
}

#[test]
fn view_blocks_are_idempotent() {
    let result = assert_stable("fn main() { view! { <div>{value}</div> }; }");
    assert!(result.contains("        <div>"));
    assert_eq!(result, "fn main() {\n    view! {\n        <div>\n            { value }\n        </div>\n    };\n}\n");
}

#[test]
fn qualified_components_keep_their_paths() {
    let result = assert_stable(
        "fn main() { view! { <leptos_router::components::Router><MyWidget \
         /></leptos_router::components::Router> }; }",
    );
    assert!(result.contains("<leptos_router::components::Router>"));
    assert!(result.contains("</leptos_router::components::Router>"));
    assert!(result.contains("<MyWidget />"));
}

#[test]
fn view_comments_remain_inside_their_macro() {
    let source = "fn main() {\n    view! {\n        <div>\n            // child\n            <span>{value /* expression */}</span>\n        </div>\n    };\n}\n";
    let result = assert_stable(source);
    assert!(result.find("<div>").unwrap() < result.find("// child").unwrap());
    assert!(result.find("// child").unwrap() < result.find("<span>").unwrap());
    assert!(result.contains("value /* expression */"));
}

#[test]
fn view_fragments_and_html_comments_survive() {
    let result = assert_stable(
        "fn main() { view! { <><span /> <!-- \"html\" --> <span /></> }; }",
    );
    assert_eq!(result.matches("<span />").count(), 2);
    assert!(result.contains("<!-- \"html\" -->"), "{result}");
}

#[test]
fn unicode_before_macro_keeps_spans_valid() {
    let result = assert_stable(
        "fn main() { let text = \"é界\"; view! { <span>{text}</span> }; }",
    );
    assert!(result.contains("\"é界\""));
    assert!(result.contains("<span>"));
}

#[test]
fn source_spans_use_unicode_columns() {
    let source = "fn main() { let text = \"é界\"; view! { <span /> }; }";
    let file = syn::parse_file(source).unwrap();
    let mut rope = crop::Rope::from(source);
    let mut edits = Vec::new();
    montrs_fmt::macro_fmt::collect_and_format_macros(
        &file,
        &rope,
        &FormatterSettings::default(),
        &mut edits,
    )
    .unwrap();
    montrs_fmt::macro_fmt::apply_edits(&mut rope, edits);
    assert_eq!(
        rope.to_string(),
        "fn main() { let text = \"é界\"; view! {\n    <span />\n}; }"
    );
}

#[test]
fn source_spans_preserve_multiline_comments() {
    let source = "fn main() { let text = \"é界\"; view! {\n    // child\n    \
                  <span />\n}; }";
    let file = syn::parse_file(source).unwrap();
    let mut rope = crop::Rope::from(source);
    let mut edits = Vec::new();
    montrs_fmt::macro_fmt::collect_and_format_macros(
        &file,
        &rope,
        &FormatterSettings::default(),
        &mut edits,
    )
    .unwrap();
    montrs_fmt::macro_fmt::apply_edits(&mut rope, edits);
    assert_eq!(rope.to_string(), source);
}

#[test]
fn test_integration_no_macros() {
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
    let settings = FormatterSettings::default();
    let result = format_source(source, &settings).expect("Formatting failed");

    assert!(result.contains("fn add(a: i32, b: i32) -> i32 {"));
    assert!(result.contains("a + b"));
}

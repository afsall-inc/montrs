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

//! Invariant tests for montrs-env.

use montrs_env::*;
use std::collections::HashMap;

#[test]
fn test_parse_env_section_simple() {
    let mut raw = HashMap::new();
    raw.insert("FOO".to_string(), toml::Value::String("bar".to_string()));
    let dirs = parse_env_section(&raw);
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].0, "FOO");
    assert!(matches!(dirs[0].1, EnvDirective::Value(ref v) if v == "bar"));
}

#[test]
fn test_parse_env_section_structured() {
    let mut table = toml::map::Map::new();
    table.insert("value".to_string(), toml::Value::String("baz".to_string()));
    table.insert("export".to_string(), toml::Value::Boolean(false));

    let mut raw = HashMap::new();
    raw.insert("FOO".to_string(), toml::Value::Table(table));
    let dirs = parse_env_section(&raw);
    assert_eq!(dirs.len(), 1);
    if let EnvDirective::Structured(ref s) = dirs[0].1 {
        assert_eq!(s.value.as_deref(), Some("baz"));
        assert_eq!(s.export, Some(false));
    } else {
        panic!("expected structured directive");
    }
}

#[test]
fn test_parse_env_section_path() {
    let mut path_table = toml::map::Map::new();
    path_table.insert(
        "path".to_string(),
        toml::Value::Array(vec![
            toml::Value::String("one".to_string()),
            toml::Value::String("two".to_string()),
        ]),
    );
    let mut raw = HashMap::new();
    raw.insert("_".to_string(), toml::Value::Table(path_table));
    let dirs = parse_env_section(&raw);
    assert_eq!(dirs.len(), 1);
    if let EnvDirective::Path(ref p) = dirs[0].1 {
        assert_eq!(p.prepend, vec!["one", "two"]);
    } else {
        panic!("expected path directive");
    }
}

#[test]
fn test_resolve_environment() {
    let mut raw = HashMap::new();
    raw.insert("A".to_string(), toml::Value::String("1".to_string()));
    raw.insert("B".to_string(), toml::Value::String("2".to_string()));
    let dirs = parse_env_section(&raw);
    let env = resolve_environment(&dirs);
    assert_eq!(env.vars.get("A"), Some(&"1".to_string()));
    assert_eq!(env.vars.len(), 2);
}

#[test]
fn test_load_dotenv() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".env");
    std::fs::write(&path, "FOO=bar\n# comment\nBAZ=qux\n").unwrap();
    let vars = load_dotenv(&path).unwrap();
    assert_eq!(vars.get("FOO"), Some(&"bar".to_string()));
    assert_eq!(vars.get("BAZ"), Some(&"qux".to_string()));
    assert_eq!(vars.len(), 2);
}

#[test]
fn test_env_diff_compute() {
    let before = HashMap::from([("A".to_string(), "1".to_string())]);
    let after = HashMap::from([
        ("A".to_string(), "2".to_string()),
        ("B".to_string(), "3".to_string()),
    ]);
    let diff = EnvDiff::compute(&before, &after);
    assert_eq!(diff.set.get("A"), Some(&"2".to_string()));
    assert_eq!(diff.set.get("B"), Some(&"3".to_string()));
    assert!(diff.unset.is_empty());
}

#[test]
fn test_env_diff_unset() {
    let before = HashMap::from([("A".to_string(), "1".to_string())]);
    let after = HashMap::new();
    let diff = EnvDiff::compute(&before, &after);
    assert!(diff.set.is_empty());
    assert_eq!(diff.unset, vec!["A"]);
}

#[test]
fn test_env_directive_serde() {
    let dir = EnvDirective::Value("hello".to_string());
    let json = serde_json::to_string(&dir).unwrap();
    assert_eq!(json, "\"hello\"");
}

#[test]
fn test_render_env_values() {
    // No templates — should pass through unchanged
    let mut raw = HashMap::new();
    raw.insert("A".to_string(), toml::Value::String("static".to_string()));
    let mut dirs = parse_env_section(&raw);
    render_env_values(&mut dirs, &HashMap::new()).unwrap();
    if let EnvDirective::Value(ref v) = dirs[0].1 {
        assert_eq!(v, "static");
    }
}

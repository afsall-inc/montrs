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

//! Invariant tests for montrs-runner.

use montrs_runner::{TaskRunner, types::*};
use std::collections::HashMap;

#[test]
fn test_task_config_simple_legacy() {
    let config = montrs_runner::TaskConfig::Simple("cargo build".to_string());
    match config {
        montrs_runner::TaskConfig::Simple(cmd) => {
            assert_eq!(cmd, "cargo build")
        }
        _ => panic!("expected Simple variant"),
    }
}

#[test]
fn test_task_config_detailed_legacy() {
    let config = montrs_runner::TaskConfig::Detailed {
        command: "cargo test".to_string(),
        description: Some("Run tests".to_string()),
        category: Some("testing".to_string()),
        dependencies: vec!["build".to_string()],
        env: HashMap::new(),
    };
    match config {
        montrs_runner::TaskConfig::Detailed {
            command,
            description,
            ..
        } => {
            assert_eq!(command, "cargo test");
            assert_eq!(description.unwrap(), "Run tests");
        }
        _ => panic!("expected Detailed variant"),
    }
}

#[test]
fn test_task_runner_empty() {
    let runner = TaskRunner::new(HashMap::new());
    assert!(runner.list().is_ok());
}

#[test]
fn test_task_runner_list() {
    let mut tasks = HashMap::new();
    tasks.insert(
        "build".to_string(),
        Task {
            name: "build".to_string(),
            command: vec![RunEntry::Script("cargo build".to_string())],
            ..Default::default()
        },
    );
    let runner = TaskRunner::new(tasks);
    assert!(runner.list().is_ok());
}

#[test]
fn test_task_runner_run_nonexistent() {
    let runner = TaskRunner::new(HashMap::new());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt.block_on(runner.run("nonexistent")).unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[test]
fn test_task_config_debug_and_clone() {
    let config = montrs_runner::TaskConfig::Simple("echo hello".to_string());
    let cloned = config.clone();
    assert_eq!(format!("{:?}", config), format!("{:?}", cloned));
}

#[test]
fn test_task_parse_from_toml_string() {
    let mut raw = HashMap::new();
    raw.insert(
        "hello".to_string(),
        toml::Value::String("echo hello".to_string()),
    );
    let tasks = montrs_runner::parser::parse_tasks_from_toml(
        raw,
        std::path::Path::new("."),
    );
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name, "hello");
    assert_eq!(tasks[0].command.len(), 1);
}

#[test]
fn test_task_dep_resolution() {
    let dep = TaskDep::Simple("build".to_string());
    assert_eq!(dep.task_name(), "build");
}

#[test]
fn test_task_output_default() {
    let outputs = TaskOutputs::default();
    assert!(matches!(outputs, TaskOutputs::Auto));
}

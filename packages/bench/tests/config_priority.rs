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

use montrs_bench::BenchConfig;
use std::{collections::HashMap, time::Duration};

#[test]
fn test_config_priority() {
    // 1. Default values
    let empty_env = HashMap::<String, String>::new();
    let config = BenchConfig::build_with_env(vec!["bench"], |key: &str| {
        empty_env.get(key).cloned()
    });
    assert_eq!(config.warmup_iterations, 10);
    assert_eq!(config.iterations, 100);
    assert_eq!(config.duration, Some(Duration::from_secs(5)));

    // 2. Env Vars
    let mut env = HashMap::new();
    env.insert("MONTRS_BENCH_WARMUP".to_string(), "30".to_string());

    let config = BenchConfig::build_with_env(vec!["bench"], |key: &str| {
        env.get(key).cloned()
    });
    assert_eq!(config.warmup_iterations, 30);

    // 3. CLI Args (Priority over Env)
    let mut env_with_val = HashMap::new();
    env_with_val.insert("MONTRS_BENCH_WARMUP".to_string(), "30".to_string());

    // We pass args explicitly
    let config = BenchConfig::build_with_env(
        vec!["bench", "--warmup", "40"],
        |key: &str| env_with_val.get(key).cloned(),
    );
    assert_eq!(config.warmup_iterations, 40); // Args win

    // 4. Duration parsing via CLI
    let config = BenchConfig::build_with_env(
        vec!["bench", "--timeout", "10"],
        |key: &str| empty_env.get(key).cloned(),
    );
    assert_eq!(config.duration, Some(Duration::from_secs(10)));

    // 5. Filter and JSON output
    let config = BenchConfig::build_with_env(
        vec![
            "bench",
            "--filter",
            "my_test",
            "--json-output",
            "report.json",
        ],
        |key: &str| empty_env.get(key).cloned(),
    );
    assert_eq!(config.filter, Some("my_test".to_string()));
    assert_eq!(config.json_output, Some("report.json".to_string()));
}


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

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};

/// Parses a duration from a string (in seconds).
fn parse_duration(arg: &str) -> Result<Duration, std::num::ParseIntError> {
    let seconds = arg.parse::<u64>()?;
    Ok(Duration::from_secs(seconds))
}

/// Internal struct for CLI argument parsing.
/// Fields are optional to allow distinguishing between "provided" and "missing".
#[derive(Parser)]
#[command(author, version, about = "MontRS Benchmark Runner", long_about = None)]
struct CliArgs {
    /// Number of warm-up iterations [default: 10].
    /// Env: MONTRS_BENCH_WARMUP
    #[arg(long = "warmup")]
    warmup_iterations: Option<u32>,

    /// Number of measurement iterations [default: 100].
    /// Env: MONTRS_BENCH_ITERATIONS
    #[arg(long = "iterations")]
    iterations: Option<u32>,

    /// Maximum duration for the benchmark in seconds (optional) [default: 5].
    /// Env: MONTRS_BENCH_TIMEOUT
    #[arg(long = "timeout", value_parser = parse_duration)]
    duration: Option<Duration>,

    /// Filter benchmarks by name.
    /// Env: MONTRS_BENCH_FILTER
    #[arg(short, long)]
    filter: Option<String>,

    /// Path to export JSON report.
    /// Env: MONTRS_BENCH_JSON_OUTPUT
    #[arg(long = "json-output")]
    json_output: Option<String>,

    /// Path to generate weights file (Substrate-style).
    /// Env: MONTRS_BENCH_GENERATE_WEIGHTS
    #[arg(long = "generate-weights")]
    generate_weights: Option<String>,
}

/// Configuration for benchmark execution.
///
/// Can be loaded from CLI arguments, environment variables `MONTRS_BENCH_*`,
/// or created programmatically.
///
/// Priority:
/// 1. Explicit arguments (if parsed via `from_args`)
/// 2. Environment variables
/// 3. Default values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchConfig {
    /// Number of warm-up iterations.
    pub warmup_iterations: u32,

    /// Number of measurement iterations.
    pub iterations: u32,

    /// Maximum duration for the benchmark in seconds (optional).
    pub duration: Option<Duration>,

    /// Filter benchmarks by name.
    pub filter: Option<String>,

    /// Path to export JSON report.
    pub json_output: Option<String>,

    /// Path to generate weights file (Substrate-style).
    pub generate_weights: Option<String>,
}

impl BenchConfig {
    /// Parses configuration from command-line arguments.
    ///
    /// This method will also respect environment variables.
    /// It falls back to defaults if neither args nor env vars are present.
    pub fn from_args() -> Self {
        let args = CliArgs::parse();
        Self::resolve(args, |key| env::var(key).ok())
    }

    /// Parses configuration from an iterator (useful for testing).
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let args = CliArgs::parse_from(itr);
        Self::resolve(args, |key| env::var(key).ok())
    }

    /// Builds configuration using a custom environment loader.
    /// Useful for testing without modifying global environment.
    pub fn build_with_env<I, T, F>(itr: I, env_loader: F) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
        F: Fn(&str) -> Option<String>,
    {
        let args = CliArgs::parse_from(itr);
        Self::resolve(args, env_loader)
    }

    /// Resolves configuration priority: Args > Env > Default
    fn resolve<F>(args: CliArgs, env_loader: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let warmup_iterations = args
            .warmup_iterations
            .or_else(|| Self::fetch_env("MONTRS_BENCH_WARMUP", &env_loader))
            .unwrap_or(10);

        let iterations = args
            .iterations
            .or_else(|| Self::fetch_env("MONTRS_BENCH_ITERATIONS", &env_loader))
            .unwrap_or(100);

        let duration = args
            .duration
            .or_else(|| {
                Self::fetch_env_string("MONTRS_BENCH_TIMEOUT", &env_loader)
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(Duration::from_secs)
            })
            .or(Some(Duration::from_secs(5)));

        let filter = args.filter.or_else(|| {
            Self::fetch_env_string("MONTRS_BENCH_FILTER", &env_loader)
        });

        let json_output = args.json_output.or_else(|| {
            Self::fetch_env_string("MONTRS_BENCH_JSON_OUTPUT", &env_loader)
        });

        let generate_weights = args.generate_weights.or_else(|| {
            Self::fetch_env_string("MONTRS_BENCH_GENERATE_WEIGHTS", &env_loader)
        });

        Self {
            warmup_iterations,
            iterations,
            duration,
            filter,
            json_output,
            generate_weights,
        }
    }

    fn fetch_env<T: std::str::FromStr, F>(
        key: &str,
        env_loader: &F,
    ) -> Option<T>
    where
        F: Fn(&str) -> Option<String>,
    {
        if let Some(val) = env_loader(key)
            && let Ok(parsed) = val.parse()
        {
            return Some(parsed);
        }
        None
    }

    fn fetch_env_string<F>(key: &str, env_loader: &F) -> Option<String>
    where
        F: Fn(&str) -> Option<String>,
    {
        env_loader(key)
    }
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 10,
            iterations: 100,
            duration: Some(Duration::from_secs(5)),
            filter: None,
            json_output: None,
            generate_weights: None,
        }
    }
}

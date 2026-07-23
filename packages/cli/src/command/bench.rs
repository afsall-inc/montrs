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

//! Bench command handler.
//!
//! Provides two modes of operation:
//! 1. **Native Mode (`--simple`)**: Benchmarks standalone files, binaries, or AppSpecs directly
//!    without the overhead of creating a temporary cargo project. It reports file size and execution speed.
//! 2. **Standard Mode**: Runs `cargo bench` within an existing project, passing filters and configuration
//!    to the underlying benchmarking harness.

use anyhow::{Context, Result};
use colored::Colorize;
use std::{fs, path::Path, process::Command, time::Instant};

#[allow(clippy::too_many_arguments)]
pub async fn run(
    target: Option<String>,
    iterations: u32,
    warmup: u32,
    timeout: Option<u64>,
    filter: Option<String>,
    json_output: Option<String>,
    simple: bool,
    generate_weights: Option<String>,
) -> Result<()> {
    if simple {
        if let Some(target_path) = &target {
            // Handle explicit targets without cargo project overhead
            return run_native_bench(
                target_path,
                iterations,
                warmup,
                generate_weights,
            )
            .await;
        } else {
            anyhow::bail!("--simple mode requires a target file or directory.");
        }
    }

    // Default behavior: run cargo bench
    run_cargo_bench(
        target,
        iterations,
        warmup,
        timeout,
        filter,
        json_output,
        generate_weights,
    )
    .await
}

async fn run_native_bench(
    target_path: &str,
    iterations: u32,
    warmup: u32,
    generate_weights: Option<String>,
) -> Result<()> {
    let path = Path::new(target_path);
    if !path.exists() {
        anyhow::bail!("Target path does not exist: {}", target_path);
    }

    // 1. Report File Size
    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len();
    let size_mb = size_bytes as f64 / 1024.0 / 1024.0;

    println!("Target: {}", target_path);
    println!("Size:   {:.2} MB ({} bytes)", size_mb, size_bytes);

    // 2. Identify Target Type
    if path.is_dir()
        || path.file_name() == Some(std::ffi::OsStr::new("montrs.toml"))
    {
        // AppSpec or Application Directory
        println!("Type:   AppSpec / Application");
        println!("Action: Benchmarking internal config load speed...");
        return bench_appspec_load(path, iterations, warmup, generate_weights)
            .await;
    }

    if path.extension().is_some_and(|e| e == "rs") {
        // Rust Source File
        println!("Type:   Rust Source");
        println!("Action: Compiling and benchmarking execution...");
        return bench_rust_source(path, iterations, warmup, generate_weights)
            .await;
    }

    // Assume Binary / Executable
    println!("Type:   Executable / Binary");
    println!("Action: Benchmarking execution speed...");
    bench_executable(path, iterations, warmup, generate_weights).await
}

/// Benchmarks loading of an AppSpec (montrs.toml).
///
/// This is an internal benchmark that measures how fast `montrs` can parse the configuration.
async fn bench_appspec_load(
    path: &Path,
    iterations: u32,
    warmup: u32,
    generate_weights: Option<String>,
) -> Result<()> {
    use montrs_bench::{report::Report, stats::BenchStats};

    // Determine the montrs.toml path
    let config_path = if path.is_dir() {
        path.join("montrs.toml")
    } else {
        path.to_path_buf()
    };

    if !config_path.exists() {
        anyhow::bail!("montrs.toml not found at {}", config_path.display());
    }

    // Warmup
    for _ in 0..warmup {
        let _ = crate::config::MontrsConfig::from_file(&config_path);
    }

    // Measure
    let mut durations = Vec::with_capacity(iterations as usize);
    let start_total = Instant::now();

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = crate::config::MontrsConfig::from_file(&config_path)?;
        durations.push(start.elapsed());
    }

    let total_duration = start_total.elapsed();
    let stats = BenchStats::new(&durations);

    println!("{}", "Results (AppSpec Load):".bold().green());
    println!("  Mean:    {:.4} ms", stats.mean * 1000.0);
    println!("  Median:  {:.4} ms", stats.median * 1000.0);
    println!("  Ops/sec: {:.2}", stats.ops_per_sec);

    if let Some(weight_path) = generate_weights {
        let mut report = Report::new();
        report.add_result(
            "appspec_load".to_string(),
            stats,
            iterations,
            total_duration.as_secs_f64(),
        );
        report.save_weights(&weight_path)?;
        println!("Weights generated at {}", weight_path.blue());
    }

    Ok(())
}

async fn bench_rust_source(
    path: &Path,
    iterations: u32,
    warmup: u32,
    generate_weights: Option<String>,
) -> Result<()> {
    // Attempt to compile using rustc to a temp binary
    // Note: This only works for standalone files without external crate dependencies (except std)
    // If the file relies on `montrs_bench`, this will fail unless we link it manually.
    // Given the constraint "no project creation", we rely on simple `rustc`.

    let temp_dir = std::env::temp_dir();
    let file_stem = path.file_stem().unwrap().to_string_lossy();
    let binary_name = if cfg!(windows) {
        format!("{}.exe", file_stem)
    } else {
        file_stem.to_string()
    };
    let binary_path = temp_dir.join(&binary_name);

    println!("Compiling {}...", path.display());
    let status = Command::new("rustc")
        .arg(path)
        .arg("-o")
        .arg(&binary_path)
        .arg("-O") // Release optimization
        .status()
        .context("Failed to invoke rustc")?;

    if !status.success() {
        println!(
            "Standard compilation failed. If this file uses external crates \
             (like montrs_bench),"
        );
        println!(
            "please run `cargo bench` within a project, or compile it \
             manually first."
        );
        anyhow::bail!("Compilation failed");
    }

    // Run the produced binary
    bench_executable(&binary_path, iterations, warmup, generate_weights).await
}

async fn bench_executable(
    path: &Path,
    iterations: u32,
    warmup: u32,
    generate_weights: Option<String>,
) -> Result<()> {
    use montrs_bench::{report::Report, stats::BenchStats};

    println!("Benchmarking execution speed...");
    // Warmup
    for _ in 0..warmup {
        let _ = Command::new(path).output()?;
    }

    // Measure
    let mut durations = Vec::with_capacity(iterations as usize);
    let start_total = Instant::now();

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = Command::new(path).output()?;
        durations.push(start.elapsed());
    }

    let total_duration = start_total.elapsed();
    let stats = BenchStats::new(&durations);

    println!("{}", "Results:".bold().green());
    println!("  Mean:    {:.4} ms", stats.mean * 1000.0);
    println!("  Median:  {:.4} ms", stats.median * 1000.0);
    println!("  P99:     {:.4} ms", stats.p99 * 1000.0);
    println!("  StdDev:  {:.4} ms", stats.std_dev * 1000.0);
    println!("  Ops/sec: {:.2}", stats.ops_per_sec);
    println!("  Total:   {:.2} s", total_duration.as_secs_f64());

    if let Some(weight_path) = generate_weights {
        let mut report = Report::new();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        report.add_result(
            name,
            stats,
            iterations,
            total_duration.as_secs_f64(),
        );
        report.save_weights(&weight_path)?;
        println!("Weights generated at {}", weight_path.blue());
    }

    Ok(())
}

async fn run_cargo_bench(
    target: Option<String>,
    iterations: u32,
    warmup: u32,
    timeout: Option<u64>,
    filter: Option<String>,
    json_output: Option<String>,
    generate_weights: Option<String>,
) -> Result<()> {
    // ... existing logic ...
    let mut cmd = Command::new("cargo");
    cmd.arg("bench");

    // Pass target as positional argument if provided (acts as a filter in cargo bench)
    if let Some(t) = target {
        cmd.arg(t);
    }

    // Arguments passed to the benchmark binary
    let mut harness_args = Vec::new();

    if let Some(f) = filter {
        cmd.arg(&f);
    }

    cmd.arg("--");

    harness_args.push(format!("--iterations={}", iterations));
    harness_args.push(format!("--warmup={}", warmup));

    if let Some(t) = timeout {
        harness_args.push(format!("--timeout={}", t));
    }

    if let Some(json) = &json_output {
        harness_args.push(format!("--json-output={}", json));
    }

    if let Some(weights) = &generate_weights {
        harness_args.push(format!("--generate-weights={}", weights));
    }

    cmd.args(&harness_args);

    // Set Env vars
    cmd.env("MONTRS_BENCH_ITERATIONS", iterations.to_string());
    cmd.env("MONTRS_BENCH_WARMUP", warmup.to_string());
    if let Some(t) = timeout {
        cmd.env("MONTRS_BENCH_TIMEOUT", t.to_string());
    }
    if let Some(json) = &json_output {
        cmd.env("MONTRS_BENCH_JSON_OUTPUT", json);
    }
    if let Some(weights) = &generate_weights {
        cmd.env("MONTRS_BENCH_GENERATE_WEIGHTS", weights);
    }

    let status = cmd.status().context("Failed to execute cargo bench")?;

    if !status.success() {
        anyhow::bail!("Benchmarks failed");
    }

    Ok(())
}


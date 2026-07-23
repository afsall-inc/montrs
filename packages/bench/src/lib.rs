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

//! Professional-grade benchmarking utilities for MontRS.
//!
//! // @agent-tool: name="bench_run" desc="Runs performance benchmarks for the project or specific targets."
//!
//! This crate provides tools for measuring performance, gathering system statistics,
//! and generating detailed reports.

pub mod config;
pub mod parameter;
pub mod report;
pub mod runner;
pub mod stats;
pub mod sys;
pub mod weights;

pub use config::BenchConfig;
use montrs_core::AgentError;
pub use parameter::{Parameter, ParametricBench};
pub use report::Report;
pub use runner::{BenchRunner, Benchmark};
use std::future::Future;
use thiserror::Error;
pub use weights::Weight;

/// Errors that can occur during benchmarking.
#[derive(Error, Debug)]
pub enum BenchError {
    #[error("Benchmark setup failed: {0}")]
    Setup(String),
    #[error("Benchmark run failed: {0}")]
    Run(String),
    #[error("Benchmark teardown failed: {0}")]
    Teardown(String),
    #[error("IO error during reporting: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl AgentError for BenchError {
    fn error_code(&self) -> &'static str {
        match self {
            BenchError::Setup(_) => "BENCH_SETUP",
            BenchError::Run(_) => "BENCH_RUN",
            BenchError::Teardown(_) => "BENCH_TEARDOWN",
            BenchError::Io(_) => "BENCH_IO",
            BenchError::Serialization(_) => "BENCH_SERIALIZATION",
        }
    }

    fn explanation(&self) -> String {
        match self {
            BenchError::Setup(e) => {
                format!("The benchmark setup phase failed: {}.", e)
            }
            BenchError::Run(e) => {
                format!("The benchmark execution phase failed: {}.", e)
            }
            BenchError::Teardown(e) => {
                format!("The benchmark teardown phase failed: {}.", e)
            }
            BenchError::Io(e) => format!(
                "An I/O error occurred while writing the benchmark report: {}.",
                e
            ),
            BenchError::Serialization(e) => {
                format!("Failed to serialize the benchmark report: {}.", e)
            }
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            BenchError::Setup(_) => vec![
                "Check the setup code for resource initialization errors."
                    .to_string(),
                "Ensure required environment variables or files are present."
                    .to_string(),
            ],
            BenchError::Run(_) => vec![
                "Debug the workload code for logic errors.".to_string(),
                "Check for race conditions if the benchmark is multi-threaded."
                    .to_string(),
            ],
            BenchError::Teardown(_) => vec![
                "Check the teardown code for resource cleanup errors."
                    .to_string(),
            ],
            BenchError::Io(_) => vec![
                "Verify that the output directory exists and is writable."
                    .to_string(),
            ],
            BenchError::Serialization(_) => vec![
                "Ensure that all data in the report is serializable to JSON."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "bench"
    }
}

/// Defines a benchmark case.
#[async_trait::async_trait]
pub trait BenchCase: Send + Sync {
    /// The name of the benchmark.
    fn name(&self) -> &str;

    /// Optional parameter info for regression testing.
    fn parameter(&self) -> Option<Parameter> {
        None
    }

    /// Set the current parameter value (if applicable).
    fn set_parameter(&self, _value: u32) {}

    /// Optional setup phase (not timed).
    async fn setup(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// The workload to measure.
    async fn run(&self) -> anyhow::Result<()>;

    /// Optional teardown phase (not timed).
    async fn teardown(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// A wrapper for simple closure-based benchmarks.
pub struct SimpleBench<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    name: String,
    func: F,
}

impl<F, Fut> SimpleBench<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    pub fn new(name: impl Into<String>, func: F) -> Self {
        Self {
            name: name.into(),
            func,
        }
    }
}

#[async_trait::async_trait]
impl<F, Fut> BenchCase for SimpleBench<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    async fn run(&self) -> anyhow::Result<()> {
        (self.func)().await
    }
}


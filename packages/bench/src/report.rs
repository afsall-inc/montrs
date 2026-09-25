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

use crate::{stats::BenchStats, sys::SystemInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A full benchmark report containing system info and results.
///
/// This structure is serializable to JSON for external analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    /// Information about the system where the benchmark was run.
    pub system: SystemInfo,
    /// A map of benchmark names to their results.
    pub results: HashMap<String, BenchResult>,
    /// The timestamp when the report was created.
    pub timestamp: time::OffsetDateTime,
}

/// The result of a single benchmark execution.
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchResult {
    /// Statistical analysis of the run.
    pub stats: BenchStats,
    /// Total number of iterations performed.
    pub iterations: u32,
    /// Total wall-clock time for all iterations (excluding warmup).
    pub total_duration_secs: f64,
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

impl Report {
    /// Creates a new, empty report with current system info.
    pub fn new() -> Self {
        Self {
            system: SystemInfo::collect(),
            results: HashMap::new(),
            timestamp: time::OffsetDateTime::now_utc(),
        }
    }

    /// Adds a result to the report.
    pub fn add_result(
        &mut self,
        name: String,
        stats: BenchStats,
        iterations: u32,
        total_duration_secs: f64,
    ) {
        self.results.insert(
            name,
            BenchResult {
                stats,
                iterations,
                total_duration_secs,
            },
        );
    }

    /// Saves the report to a JSON file.
    pub fn save_json(&self, path: &str) -> anyhow::Result<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    /// Generates a structured JSON baseline file for regression gating.
    pub fn save_baselines(
        &self,
        path: &str,
        device: &str,
    ) -> anyhow::Result<()> {
        let mut store = if std::path::Path::new(path).exists() {
            let content = std::fs::read_to_string(path)?;
            serde_json::from_str::<crate::budget::BaselineStore>(&content)
                .unwrap_or_default()
        } else {
            crate::budget::BaselineStore::default()
        };

        for (name, result) in &self.results {
            store.insert(crate::budget::BenchmarkBaseline {
                name: name.clone(),
                device: device.to_string(),
                p50_ms: result.stats.median * 1000.0,
                p95_ms: result.stats.p95 * 1000.0,
                p99_ms: result.stats.p99 * 1000.0,
                ops_per_sec: result.stats.ops_per_sec,
            });
        }

        let json = serde_json::to_string_pretty(&store)?;
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)?;
        Ok(())
    }
}

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

use serde::{Deserialize, Serialize};
use statrs::statistics::{Data, Distribution, Max, Min, OrderStatistics};
use std::time::Duration;

/// Statistical analysis of benchmark results.
///
/// This struct holds key performance metrics calculated from a series of measurements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchStats {
    /// Arithmetic mean of execution times (seconds).
    pub mean: f64,
    /// Median execution time (seconds).
    pub median: f64,
    /// Minimum execution time observed (seconds).
    pub min: f64,
    /// Maximum execution time observed (seconds).
    pub max: f64,
    /// Standard deviation of execution times.
    pub std_dev: f64,
    /// 95th percentile execution time.
    pub p95: f64,
    /// 99th percentile execution time.
    pub p99: f64,
    /// Estimated operations per second (throughput).
    pub ops_per_sec: f64,
    /// Linear regression slope (time per unit of parameter).
    pub slope: Option<f64>,
    /// Linear regression intercept (base time).
    pub intercept: Option<f64>,
}

impl BenchStats {
    /// Calculates statistics from a slice of durations.
    pub fn new(durations: &[Duration]) -> Self {
        Self::with_params(durations, None)
    }

    /// Calculates statistics with optional parameter values for regression.
    pub fn with_params(durations: &[Duration], params: Option<&[u32]>) -> Self {
        let mut data: Vec<f64> =
            durations.iter().map(|d| d.as_secs_f64()).collect();
        let mut stats_data = Data::new(&mut data);

        let mean = stats_data.mean().unwrap_or(0.0);
        let std_dev = stats_data.std_dev().unwrap_or(0.0);
        let median = stats_data.median();
        let min = stats_data.min();
        let max = stats_data.max();
        let p95 = stats_data.percentile(95);
        let p99 = stats_data.percentile(99);

        let ops_per_sec = if mean > 0.0 { 1.0 / mean } else { 0.0 };

        let mut slope = None;
        let mut intercept = None;

        if let Some(p_vals) = params {
            if p_vals.len() == durations.len() && p_vals.len() > 1 {
                // Simple linear regression calculation
                let x: Vec<f64> = p_vals.iter().map(|&v| v as f64).collect();
                let y: Vec<f64> = data;

                let n = x.len() as f64;
                let sum_x: f64 = x.iter().sum();
                let sum_y: f64 = y.iter().sum();
                let sum_xy: f64 =
                    x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
                let sum_xx: f64 = x.iter().map(|xi| xi * xi).sum();

                let denom = n * sum_xx - sum_x * sum_x;
                if denom != 0.0 {
                    let m = (n * sum_xy - sum_x * sum_y) / denom;
                    let b = (sum_y - m * sum_x) / n;
                    slope = Some(m);
                    intercept = Some(b);
                }
            } else {
                // data was moved into y if params exists but validation fails, but here we don't strictly need it back
            }
        }

        Self {
            mean,
            median,
            min,
            max,
            std_dev,
            p95,
            p99,
            ops_per_sec,
            slope,
            intercept,
        }
    }
}

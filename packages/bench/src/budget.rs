// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Performance budgets and regression baselines for MontRS benchmarks.
//!
//! Rather than generating blockchain-inspired weights or opaque linear constants,
//! MontRS tracks explicit performance budgets (e.g. latency ceilings) and
//! saves structured JSON baselines to `.montrs/bench.json` to detect
//! regressions deterministically across devices.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Declared performance budget for a target, route, or component.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Budget {
    /// Maximum allowed P95 execution time.
    pub max_p95_ms: f64,
    /// Maximum allowed median (P50) execution time.
    pub max_p50_ms: Option<f64>,
    /// Minimum allowed operations per second.
    pub min_ops_per_sec: Option<f64>,
}

impl Budget {
    pub fn p95(max_ms: f64) -> Self {
        Self {
            max_p95_ms: max_ms,
            max_p50_ms: None,
            min_ops_per_sec: None,
        }
    }
}

/// A stored baseline measurement for a benchmark case on a specific device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkBaseline {
    pub name: String,
    pub device: String,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub ops_per_sec: f64,
}

/// Collection of baselines across multiple benchmark cases and devices.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BaselineStore {
    pub baselines: BTreeMap<String, BenchmarkBaseline>,
}

impl BaselineStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, baseline: BenchmarkBaseline) {
        let key = format!("{}:{}", baseline.device, baseline.name);
        self.baselines.insert(key, baseline);
    }

    pub fn get(&self, device: &str, name: &str) -> Option<&BenchmarkBaseline> {
        let key = format!("{}:{}", device, name);
        self.baselines.get(&key)
    }

    pub fn check_regression(
        &self,
        current: &BenchmarkBaseline,
        tolerance_pct: f64,
    ) -> Result<(), String> {
        if let Some(baseline) = self.get(&current.device, &current.name) {
            let max_allowed = baseline.p95_ms * (1.0 + tolerance_pct / 100.0);
            if current.p95_ms > max_allowed {
                return Err(format!(
                    "Performance regression in '{}' on device '{}': P95 is \
                     {:.2}ms, baseline is {:.2}ms (allowed: {:.2}ms with \
                     {:.1}% tolerance)",
                    current.name,
                    current.device,
                    current.p95_ms,
                    baseline.p95_ms,
                    max_allowed,
                    tolerance_pct
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_store_detects_regression() {
        let mut store = BaselineStore::new();
        store.insert(BenchmarkBaseline {
            name: "render".to_string(),
            device: "desktop".to_string(),
            p50_ms: 10.0,
            p95_ms: 15.0,
            p99_ms: 20.0,
            ops_per_sec: 100.0,
        });

        // 10% tolerance allows up to 16.5ms
        let pass = BenchmarkBaseline {
            name: "render".to_string(),
            device: "desktop".to_string(),
            p50_ms: 11.0,
            p95_ms: 16.0,
            p99_ms: 21.0,
            ops_per_sec: 95.0,
        };
        assert!(store.check_regression(&pass, 10.0).is_ok());

        let fail = BenchmarkBaseline {
            name: "render".to_string(),
            device: "desktop".to_string(),
            p50_ms: 12.0,
            p95_ms: 18.0,
            p99_ms: 25.0,
            ops_per_sec: 80.0,
        };
        assert!(store.check_regression(&fail, 10.0).is_err());
    }
}

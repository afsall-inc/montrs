// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! In-process synthetic traffic and stress testing.
//!
//! [`Traffic`] drives a [`TestClient`] or an async closure with N synthetic
//! requests, recording status distribution, error count, and p50/p95/p99
//! latencies.
//!
//! It runs **in-process without sockets** so benchmarks and stress checks are
//! hermetic and repeatable.

use crate::http::{TestClient, TestResponse};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Presets for synthetic traffic patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficProfile {
    /// Quick smoke test: 50 requests.
    Smoke,
    /// Burst/spike test: 200 rapid requests.
    Spike,
    /// Soak/endurance check: 1000 requests.
    Soak,
}

impl TrafficProfile {
    /// The number of requests for this profile.
    pub fn requests(&self) -> usize {
        match self {
            Self::Smoke => 50,
            Self::Spike => 200,
            Self::Soak => 1000,
        }
    }
}

/// Statistics collected from a synthetic traffic run.
#[derive(Debug, Clone)]
pub struct TrafficReport {
    /// Total completed requests.
    pub total: usize,
    /// Succeeded requests (status 2xx).
    pub successful: usize,
    /// Failed requests (status >= 400 or render error).
    pub failed: usize,
    /// Count per HTTP status code.
    pub status_counts: BTreeMap<u16, usize>,
    /// Sorted execution latencies.
    pub latencies: Vec<Duration>,
    /// Total wall-clock duration of the run.
    pub total_duration: Duration,
}

impl TrafficReport {
    /// Median latency (p50).
    pub fn p50(&self) -> Duration {
        self.percentile(50.0)
    }

    /// 95th percentile latency (p95).
    pub fn p95(&self) -> Duration {
        self.percentile(95.0)
    }

    /// 99th percentile latency (p99).
    pub fn p99(&self) -> Duration {
        self.percentile(99.0)
    }

    /// Maximum latency observed.
    pub fn max(&self) -> Duration {
        self.latencies.last().copied().unwrap_or_default()
    }

    /// Requests completed per second of wall-clock time.
    pub fn reqs_per_sec(&self) -> f64 {
        let secs = self.total_duration.as_secs_f64();
        if secs > 0.0 {
            self.total as f64 / secs
        } else {
            0.0
        }
    }

    fn percentile(&self, p: f64) -> Duration {
        if self.latencies.is_empty() {
            return Duration::ZERO;
        }
        let idx = (((self.latencies.len() as f64) * (p / 100.0)).ceil()
            as usize)
            .saturating_sub(1)
            .min(self.latencies.len() - 1);
        self.latencies[idx]
    }

    /// Assert all requests succeeded (status 2xx) and p95 latency is below `max_p95`.
    #[track_caller]
    pub fn assert_ok(&self, max_p95: Duration) {
        assert_eq!(
            self.failed, 0,
            "expected 0 failed requests, found {} (status counts: {:?})",
            self.failed, self.status_counts
        );
        assert!(
            self.p95() <= max_p95,
            "p95 latency {:?} exceeded budget {:?}",
            self.p95(),
            max_p95
        );
    }
}

/// A synthetic traffic generator.
pub struct Traffic {
    count: usize,
}

impl Traffic {
    /// Run `count` synthetic requests.
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    /// Run with a predefined profile.
    pub fn profile(profile: TrafficProfile) -> Self {
        Self::new(profile.requests())
    }

    /// Execute against a [`TestClient`] using `make_uri`.
    pub fn run_client<F>(
        &self,
        client: &TestClient,
        mut make_uri: F,
    ) -> TrafficReport
    where
        F: FnMut(usize) -> String,
    {
        self.run_raw(|i| {
            let uri = make_uri(i);
            client.get(&uri)
        })
    }

    /// Run with a custom in-process request closure.
    pub fn run_raw<F>(&self, mut request_fn: F) -> TrafficReport
    where
        F: FnMut(usize) -> Result<TestResponse, Box<dyn std::error::Error>>,
    {
        let mut status_counts: BTreeMap<u16, usize> = BTreeMap::new();
        let mut latencies: Vec<Duration> = Vec::with_capacity(self.count);
        let mut successful = 0;
        let mut failed = 0;

        let start_all = Instant::now();
        for i in 0..self.count {
            let start = Instant::now();
            let res = request_fn(i);
            let elapsed = start.elapsed();
            latencies.push(elapsed);

            match res {
                Ok(resp) => {
                    *status_counts.entry(resp.status).or_insert(0) += 1;
                    if resp.is_success() {
                        successful += 1;
                    } else {
                        failed += 1;
                    }
                }
                Err(_) => {
                    *status_counts.entry(500).or_insert(0) += 1;
                    failed += 1;
                }
            }
        }
        let total_duration = start_all.elapsed();

        latencies.sort();

        TrafficReport {
            total: self.count,
            successful,
            failed,
            status_counts,
            latencies,
            total_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_traffic_computes_percentiles() {
        let traffic = Traffic::new(50);
        let report = traffic.run_raw(|i| {
            Ok(TestResponse {
                status: if i % 10 == 0 { 404 } else { 200 },
                headers: Vec::new(),
                body: b"ok".to_vec(),
            })
        });

        assert_eq!(report.total, 50);
        assert_eq!(report.successful, 45);
        assert_eq!(report.failed, 5);
        assert_eq!(report.status_counts.get(&200), Some(&45));
        assert_eq!(report.status_counts.get(&404), Some(&5));
        assert!(report.p50() <= report.p95());
        assert!(report.p95() <= report.p99());
        assert!(report.p99() <= report.max());
        assert!(report.reqs_per_sec() > 0.0);
    }

    #[test]
    fn smoke_profile_assert_ok() {
        let traffic = Traffic::profile(TrafficProfile::Smoke);
        let report = traffic.run_raw(|_| {
            Ok(TestResponse {
                status: 200,
                headers: Vec::new(),
                body: b"success".to_vec(),
            })
        });
        report.assert_ok(Duration::from_secs(1));
    }
}

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
        let mut data: Vec<f64> = durations.iter().map(|d| d.as_secs_f64()).collect();
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
                let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
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

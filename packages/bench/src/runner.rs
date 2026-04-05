use crate::BenchCase;
use crate::config::BenchConfig;
use crate::report::Report;
use crate::stats::BenchStats;
use colored::*;
use std::time::Instant;

/// The main entry point for running benchmarks.
///
/// `BenchRunner` orchestrates the execution of registered benchmarks.
/// It handles configuration, setup/teardown, warmup, measurement, and reporting.
///
/// # Example
///
/// ```rust
/// use montrs_bench::{BenchRunner, Benchmark};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut runner = BenchRunner::from_args();
///     runner.add(Benchmark::new("my_bench", || async {
///         // ... workload ...
///         Ok(())
///     }));
///     runner.run().await
/// }
/// ```
pub struct BenchRunner {
    config: BenchConfig,
    benchmarks: Vec<Box<dyn BenchCase>>,
}

impl Default for BenchRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchRunner {
    /// Creates a new `BenchRunner` with configuration parsed from command-line arguments and environment variables.
    ///
    /// This is the recommended constructor for benchmark binaries.
    pub fn from_args() -> Self {
        let config = BenchConfig::from_args();
        Self::log_config(&config);
        Self {
            config,
            benchmarks: Vec::new(),
        }
    }

    /// Creates a new `BenchRunner` with default configuration (ignoring CLI args, but using defaults).
    ///
    /// Useful for programmatic usage where CLI args should not be parsed.
    pub fn new() -> Self {
        Self {
            config: BenchConfig::default(),
            benchmarks: Vec::new(),
        }
    }

    /// Creates a new `BenchRunner` with custom configuration.
    pub fn with_config(config: BenchConfig) -> Self {
        Self::log_config(&config);
        Self {
            config,
            benchmarks: Vec::new(),
        }
    }

    fn log_config(config: &BenchConfig) {
        println!("{}", "Configuration:".bold().blue());
        println!("  Warmup:     {} iterations", config.warmup_iterations);
        println!("  Iterations: {}", config.iterations);
        if let Some(d) = config.duration {
            println!("  Timeout:    {:?}", d);
        } else {
            println!("  Timeout:    None");
        }
        if let Some(f) = &config.filter {
            println!("  Filter:     {}", f);
        }
        if let Some(j) = &config.json_output {
            println!("  JSON Out:   {}", j);
        }
        println!("---------------------------------------------------");
    }

    /// Adds a benchmark to the runner.
    pub fn add<B: BenchCase + 'static>(&mut self, benchmark: B) -> &mut Self {
        self.benchmarks.push(Box::new(benchmark));
        self
    }

    /// Executes all registered benchmarks and reports results.
    ///
    /// This method will:
    /// 1. Print system information.
    /// 2. Iterate through benchmarks (filtering if configured).
    /// 3. Run setup, warmup, measurement loop, and teardown for each.
    /// 4. Print results to stdout.
    /// 5. Optionally save a JSON report.
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut report = Report::new();

        println!("{}", "Running MontRS Benchmarks".bold().green());
        println!(
            "System: {} ({})",
            report.system.os_name, report.system.cpu_brand
        );
        if let Some(size) = report.system.binary_size_bytes {
            let size_mb = size as f64 / 1024.0 / 1024.0;
            println!("Binary Size: {:.2} MB", size_mb);
        }
        println!("---------------------------------------------------");

        for bench in &self.benchmarks {
            if let Some(filter) = &self.config.filter
                && !bench.name().contains(filter)
            {
                continue;
            }

            self.run_single_bench(bench.as_ref(), &mut report).await?;
        }

        if let Some(path) = &self.config.json_output {
            report.save_json(path)?;
            println!("Report saved to {}", path.blue());
        }

        if let Some(path) = &self.config.generate_weights {
            report.save_weights(path)?;
            println!("Weights generated at {}", path.blue());
        }

        Ok(())
    }

    /// Internal method to run a single benchmark case.
    async fn run_single_bench(
        &self,
        bench: &dyn BenchCase,
        report: &mut Report,
    ) -> anyhow::Result<()> {
        print!("Running {}... ", bench.name().cyan());

        bench.setup().await?;

        // Warmup
        for _ in 0..self.config.warmup_iterations {
            bench.run().await?;
        }

        let mut durations = Vec::with_capacity(self.config.iterations as usize);
        let start_total = Instant::now();

        // Check for parameter support (Substrate-inspired parametric benchmarking)
        let param_info = bench.parameter();
        let mut params_used = Vec::new();

        if let Some(param) = &param_info {
            // Parametric mode: Iterate through parameter values
            let values = param.values();
            let runs_per_val = std::cmp::max(1, self.config.iterations / values.len() as u32);

            println!(
                "  Parameter: {} ({} values, {} runs/val)",
                param.name,
                values.len(),
                runs_per_val
            );

            for &val in &values {
                bench.set_parameter(val);
                // Mini-warmup for new param
                bench.run().await?;

                for _ in 0..runs_per_val {
                    let start = Instant::now();
                    bench.run().await?;
                    durations.push(start.elapsed());
                    params_used.push(val);
                }
            }
        } else {
            // Standard mode
            for _ in 0..self.config.iterations {
                let start = Instant::now();
                bench.run().await?;
                durations.push(start.elapsed());

                if let Some(max_dur) = self.config.duration
                    && start_total.elapsed() > max_dur
                {
                    break;
                }
            }
        }

        let total_duration = start_total.elapsed();
        bench.teardown().await?;

        let stats = if !params_used.is_empty() {
            BenchStats::with_params(&durations, Some(&params_used))
        } else {
            BenchStats::new(&durations)
        };

        report.add_result(
            bench.name().to_string(),
            stats.clone(),
            durations.len() as u32,
            total_duration.as_secs_f64(),
        );

        println!("{}", "Done".green());
        println!("  Mean:    {:.4} µs", stats.mean * 1_000_000.0);
        println!("  Median:  {:.4} µs", stats.median * 1_000_000.0);
        println!("  P99:     {:.4} µs", stats.p99 * 1_000_000.0);
        println!("  StdDev:  {:.4} µs", stats.std_dev * 1_000_000.0);
        if let Some(slope) = stats.slope {
            println!("  Slope:   {:.4} ns/param", slope * 1_000_000_000.0);
        }
        if let Some(intercept) = stats.intercept {
            println!("  Intercept: {:.4} µs", intercept * 1_000_000.0);
        }
        println!("  Ops/sec: {:.2}", stats.ops_per_sec);
        println!("---------------------------------------------------");

        Ok(())
    }
}

/// A convenience wrapper to define benchmarks using the `BenchCase` trait.
pub struct Benchmark<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    name: String,
    func: F,
}

impl<F, Fut> Benchmark<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    pub fn new(name: &str, func: F) -> Self {
        Self {
            name: name.to_string(),
            func,
        }
    }
}

#[async_trait::async_trait]
impl<F, Fut> BenchCase for Benchmark<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    async fn run(&self) -> anyhow::Result<()> {
        (self.func)().await
    }
}

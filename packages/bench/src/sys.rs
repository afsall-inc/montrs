use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

/// Captures system information for the benchmark report.
///
/// This provides context for benchmark results, allowing comparison across different environments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating System name (e.g., "Windows", "Linux").
    pub os_name: String,
    /// Operating System version.
    pub os_version: String,
    /// Kernel version.
    pub kernel_version: String,
    /// Hostname of the machine.
    pub host_name: String,
    /// CPU brand/model string.
    pub cpu_brand: String,
    /// Number of physical CPU cores.
    pub cpu_cores: usize,
    /// Total system memory in Gigabytes.
    pub total_memory_gb: f64,
    /// CPU Architecture (e.g., "x86_64", "aarch64").
    pub cpu_arch: String,
    /// CPU Frequency in MHz.
    pub cpu_frequency_mhz: u64,
    /// Number of physical CPU cores.
    pub physical_cores: usize,
    /// Version of the Rust compiler used to build the benchmark.
    pub rust_version: String,
    /// Size of the benchmark binary in bytes.
    pub binary_size_bytes: Option<u64>,
}

impl SystemInfo {
    /// Collects system information from the current environment.
    pub fn collect() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        sys.refresh_all();

        let binary_size_bytes = std::env::current_exe()
            .ok()
            .and_then(|path| std::fs::metadata(path).ok())
            .map(|meta| meta.len());

        let cpu = sys.cpus().first();

        Self {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version()
                .unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version()
                .unwrap_or_else(|| "Unknown".to_string()),
            host_name: System::host_name()
                .unwrap_or_else(|| "Unknown".to_string()),
            cpu_brand: cpu.map(|c| c.brand().to_string()).unwrap_or_default(),
            cpu_cores: sys.cpus().len(),
            cpu_arch: System::cpu_arch()
                .unwrap_or_else(|| "Unknown".to_string()),
            cpu_frequency_mhz: cpu.map(|c| c.frequency()).unwrap_or(0),
            physical_cores: sys
                .physical_core_count()
                .unwrap_or(sys.cpus().len()),
            total_memory_gb: sys.total_memory() as f64
                / 1024.0
                / 1024.0
                / 1024.0,
            rust_version: rustc_version_runtime::version().to_string(),
            binary_size_bytes,
        }
    }
}

//! E2E test command.
//!
//! This plate runs the full end-to-end testing pipeline. It coordinates:
//! 1. Building the application.
//! 2. Starting the backend server.
//! 3. Running the E2E test suite against the running server.

use montrs_build::Pipeline;
use std::path::Path;

/// Executes the E2E tests.
pub async fn run(
    headless: bool,
    keep_alive: bool,
    browser: Option<String>,
) -> anyhow::Result<()> {
    let pipeline = Pipeline::from_root(Path::new("."))?;

    // Build everything
    pipeline.build_all()?;

    // Set environment variables for runtime configuration
    unsafe {
        std::env::set_var("MONTRS_E2E_HEADLESS", headless.to_string());
        if keep_alive {
            std::env::set_var("MONTRS_E2E_KEEP_ALIVE", "true");
        }
        if let Some(b) = browser {
            std::env::set_var("MONTRS_E2E_BROWSER", b);
        }
    }

    // Run E2E tests
    let status = std::process::Command::new("cargo")
        .args(["test", "--package", "e2e"])
        .status()?;
    if !status.success() {
        anyhow::bail!("E2E tests failed");
    }
    Ok(())
}

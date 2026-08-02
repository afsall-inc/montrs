//! E2E test command.

pub async fn run(
    headless: bool,
    keep_alive: bool,
    browser: Option<String>,
) -> anyhow::Result<()> {
    let _ = headless;
    let _ = keep_alive;
    let _ = browser;

    // Build the app first, then run Playwright tests
    let pipeline = montrs_build::Pipeline::from_root(std::path::Path::new("."))?;
    pipeline.build_all()?;

    // Run the E2E tests
    println!(" Running E2E tests...");
    let status = std::process::Command::new("cargo")
        .args(["test", "--package", "e2e"])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("E2E tests failed");
    }

    Ok(())
}
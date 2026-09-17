use montrs_test::e2e::MontrsDriver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize MontrsDriver with default config (reads from env vars).
    let driver = MontrsDriver::new().await?;

    // Navigate to the app (automatically handles the base URL).
    driver.goto("/").await?;

    println!("Successfully navigated to {}", driver.url());

    driver.close().await;

    Ok(())
}

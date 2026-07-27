use crate::config::MontrsConfig;
use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    let _config = MontrsConfig::load().unwrap_or_default();

    println!("Upgrading montrs...");

    let mut cmd = Command::new("cargo");
    cmd.args(["install", "montrs"]);

    let status = cmd.status()?;

    if status.success() {
        println!("Successfully upgraded montrs!");
    } else {
        anyhow::bail!("Failed to upgrade montrs");
    }
    Ok(())
}

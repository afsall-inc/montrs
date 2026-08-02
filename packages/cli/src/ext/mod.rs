use anyhow::Context;
use std::process::Command;

pub fn exe_command(cmd: &mut Command) -> anyhow::Result<()> {
    let status = cmd.status().context("Failed to execute command")?;
    if !status.success() {
        anyhow::bail!("Command failed with status: {}", status);
    }
    Ok(())
}

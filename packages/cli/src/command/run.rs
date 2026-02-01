use crate::config::MontrsConfig;
use montrs_runner::TaskRunner;
use std::process::Command;

pub async fn run(task_name: String) -> anyhow::Result<()> {
    // Check for Mise configuration files
    let mise_configs = ["mise.toml", "mise.local.toml", ".mise.toml", ".mise.local.toml"];
    let has_mise_config = mise_configs.iter().any(|f| std::path::Path::new(f).exists());

    if has_mise_config {
        // Try to delegate to mise
        let status = Command::new("mise")
            .arg("run")
            .arg(&task_name)
            .status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            Ok(_) => anyhow::bail!("Mise task '{}' failed", task_name),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Mise not installed, fall back to internal runner
                eprintln!("Mise config found but 'mise' command not found. Falling back to internal runner...");
            }
            Err(e) => anyhow::bail!("Failed to execute mise: {}", e),
        }
    }

    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(config.tasks);
    runner.run(&task_name).await?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    // Check for Mise configuration files
    let mise_configs = ["mise.toml", "mise.local.toml", ".mise.toml", ".mise.local.toml"];
    let has_mise_config = mise_configs.iter().any(|f| std::path::Path::new(f).exists());

    if has_mise_config {
        // Try to list tasks from mise
        let status = Command::new("mise")
            .arg("tasks")
            .status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            Ok(_) => {
                // If mise tasks fails, we'll fall back to internal runner's list
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Mise not installed, fall back to internal runner
            }
            Err(e) => anyhow::bail!("Failed to execute mise: {}", e),
        }
    }

    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(config.tasks);
    runner.list()?;
    Ok(())
}

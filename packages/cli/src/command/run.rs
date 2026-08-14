use crate::config::MontrsConfig;
use montrs_runner::TaskRunner;
use std::path::Path;

pub async fn run(task_name: String) -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner =
        TaskRunner::from_config_tasks(config.meta.tasks, Path::new("."));
    runner.run(&task_name).await?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner =
        TaskRunner::from_config_tasks(config.meta.tasks, Path::new("."));
    runner.list()?;
    Ok(())
}

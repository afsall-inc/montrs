use crate::config::MontrsConfig;
use montrs_runner::TaskRunner;

pub async fn run(task_name: String) -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(config.tasks);
    runner.run(&task_name).await?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(config.tasks);
    runner.list()?;
    Ok(())
}

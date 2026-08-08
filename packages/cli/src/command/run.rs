use crate::config::MontrsConfig;
use montrs_runner::{TaskConfig, TaskRunner};
use std::collections::HashMap;

/// Convert raw `serde_json::Value` task definitions into `TaskConfig`.
fn to_task_configs(
    tasks: HashMap<String, serde_json::Value>,
) -> HashMap<String, TaskConfig> {
    tasks
        .into_iter()
        .filter_map(|(name, value)| {
            let config = if value.is_string() {
                TaskConfig::Simple(
                    value.as_str().unwrap_or_default().to_string(),
                )
            } else {
                serde_json::from_value(value).ok()?
            };
            Some((name, config))
        })
        .collect()
}

pub async fn run(task_name: String) -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(to_task_configs(config.meta.tasks));
    runner.run(&task_name).await?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner = TaskRunner::new(to_task_configs(config.meta.tasks));
    runner.list()?;
    Ok(())
}

pub mod executor;
pub mod parser;
pub mod scheduler;
pub mod template;
pub mod types;
pub mod workspace;

// Backward-compatibility: `TaskRunner` orchestration wrapper.
use std::collections::HashMap;
pub use types::*;

/// A simple wrapper around a task map, preserving the legacy API.
#[derive(Default)]
pub struct TaskRunner {
    tasks: HashMap<String, Task>,
}

impl TaskRunner {
    pub fn new(tasks: HashMap<String, Task>) -> Self {
        Self { tasks }
    }

    pub fn from_config_tasks(
        tasks: HashMap<String, toml::Value>,
        config_root: &std::path::Path,
    ) -> Self {
        let parsed = crate::parser::parse_tasks_from_toml(tasks, config_root);
        let mut map = HashMap::new();
        for task in parsed {
            map.insert(task.name.clone(), task);
        }
        Self { tasks: map }
    }

    pub async fn run(&self, task_name: &str) -> anyhow::Result<()> {
        let all_tasks: Vec<Task> = self.tasks.values().cloned().collect();
        let task = self.tasks.get(task_name).ok_or_else(|| {
            anyhow::anyhow!("Task '{}' not found in configuration", task_name)
        })?;
        let config = executor::TaskExecutorConfig::default();
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(4));
        executor::execute_task(task, &all_tasks, &config, semaphore).await?;
        Ok(())
    }

    pub fn list(&self) -> anyhow::Result<()> {
        if self.tasks.is_empty() {
            println!("No tasks defined");
            return Ok(());
        }
        println!("{}", console::style("Available Tasks:").bold());
        let mut names: Vec<&String> = self.tasks.keys().collect();
        names.sort();
        for name in names {
            let desc = self
                .tasks
                .get(name)
                .map(|t| t.description.clone())
                .unwrap_or_default();
            println!(
                "    {:<15} {}",
                console::style(name).cyan(),
                console::style(desc).dim()
            );
        }
        Ok(())
    }
}

/// Legacy `TaskConfig` enum — kept for backward compatibility.
#[derive(Debug, Clone)]
pub enum TaskConfig {
    Simple(String),
    Detailed {
        command: String,
        description: Option<String>,
        category: Option<String>,
        dependencies: Vec<String>,
        env: HashMap<String, String>,
    },
}

impl From<TaskConfig> for Task {
    fn from(config: TaskConfig) -> Self {
        match config {
            TaskConfig::Simple(cmd) => Task {
                command: vec![crate::types::RunEntry::Script(cmd)],
                ..Default::default()
            },
            TaskConfig::Detailed {
                command,
                description,
                category: _,
                dependencies,
                env,
            } => {
                let mut task = Task {
                    command: vec![crate::types::RunEntry::Script(command)],
                    description: description.unwrap_or_default(),
                    ..Default::default()
                };
                task.depends = dependencies
                    .into_iter()
                    .map(crate::types::TaskDep::Simple)
                    .collect();
                for (k, v) in env {
                    task.env.insert(k, v);
                }
                task
            }
        }
    }
}

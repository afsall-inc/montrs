use anyhow::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    process::Command,
};

/// Configuration for custom tasks.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum TaskConfig {
    /// A simple command string.
    Simple(String),
    /// A detailed task definition.
    Detailed {
        /// The command to execute.
        command: String,
        /// Description of the task.
        #[serde(default)]
        description: Option<String>,
        /// Category for grouping tasks.
        #[serde(default)]
        category: Option<String>,
        /// List of dependent tasks to run before this one.
        #[serde(default)]
        dependencies: Vec<String>,
        /// Environment variables to set for this task.
        #[serde(default)]
        env: HashMap<String, String>,
    },
}

pub struct TaskRunner {
    tasks: HashMap<String, TaskConfig>,
}

impl TaskRunner {
    pub fn new(tasks: HashMap<String, TaskConfig>) -> Self {
        Self { tasks }
    }

    pub async fn run(&self, task_name: &str) -> Result<()> {
        let mut executed = HashSet::new();
        self.execute_task_recursive(task_name, &mut executed)
            .await?;
        Ok(())
    }

    async fn execute_task_recursive(
        &self,
        name: &str,
        executed: &mut HashSet<String>,
    ) -> Result<()> {
        if executed.contains(name) {
            return Ok(());
        }

        let task = self.tasks.get(name).ok_or_else(|| {
            anyhow::anyhow!("Task '{}' not found in configuration", name)
        })?;

        // 1. Run dependencies first
        if let TaskConfig::Detailed { dependencies, .. } = task {
            for dep in dependencies {
                Box::pin(self.execute_task_recursive(dep, executed)).await?;
            }
        }

        // 2. Run the task itself
        println!(
            "{} Running task: {}",
            style("🛠").bold(),
            style(name).cyan().bold()
        );

        match task {
            TaskConfig::Simple(cmd_str) => {
                self.run_shell_cmd(cmd_str, &HashMap::new())?;
            }
            TaskConfig::Detailed {
                command,
                env,
                description,
                ..
            } => {
                if let Some(desc) = description {
                    println!("   {}", style(desc).italic().dim());
                }
                self.run_shell_cmd(command, env)?;
            }
        }

        executed.insert(name.to_string());
        Ok(())
    }

    fn run_shell_cmd(
        &self,
        cmd_str: &str,
        env_vars: &HashMap<String, String>,
    ) -> Result<()> {
        #[cfg(windows)]
        let mut cmd = Command::new("powershell");
        #[cfg(windows)]
        cmd.arg("-Command");

        #[cfg(not(windows))]
        let mut cmd = Command::new("sh");
        #[cfg(not(windows))]
        cmd.arg("-c");

        cmd.arg(cmd_str);

        for (key, val) in env_vars {
            cmd.env(key, val);
        }

        let status = cmd.status().context("Failed to execute command")?;
        if !status.success() {
            anyhow::bail!("Command failed with status: {}", status);
        }
        Ok(())
    }

    pub fn list(&self) -> Result<()> {
        if self.tasks.is_empty() {
            println!("No tasks defined");
            return Ok(());
        }

        println!("{}", style("Available Tasks:").bold());

        let mut categories: HashMap<String, Vec<(&String, &TaskConfig)>> =
            HashMap::new();
        for (name, task) in &self.tasks {
            let cat = match task {
                TaskConfig::Detailed { category, .. } => {
                    category.clone().unwrap_or_else(|| "Other".to_string())
                }
                _ => "Other".to_string(),
            };
            categories.entry(cat).or_default().push((name, task));
        }

        let mut sorted_cats: Vec<_> = categories.keys().collect();
        sorted_cats.sort();

        for cat in sorted_cats {
            println!("\n  {}", style(cat).yellow().bold());
            let mut tasks = categories.get(cat).unwrap().clone();
            tasks.sort_by(|a, b| a.0.cmp(b.0));

            for (name, task) in tasks {
                let desc = match task {
                    TaskConfig::Detailed { description, .. } => {
                        description.clone().unwrap_or_default()
                    }
                    _ => String::new(),
                };
                println!(
                    "    {:<15} {}",
                    style(name).cyan(),
                    style(desc).dim()
                );
            }
        }

        Ok(())
    }
}

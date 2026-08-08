use crate::types::{RunEntry, Task, TaskOutput};
use crate::scheduler::task_needs_permit;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

/// Configuration for task execution.
#[derive(Clone)]
pub struct TaskExecutorConfig {
    pub force: bool,
    pub cd: Option<std::path::PathBuf>,
    pub shell: Option<String>,
    pub timings: bool,
    pub continue_on_error: bool,
    pub dry_run: bool,
    pub skip_deps: bool,
}

impl Default for TaskExecutorConfig {
    fn default() -> Self {
        Self {
            force: false,
            cd: None,
            shell: None,
            timings: false,
            continue_on_error: false,
            dry_run: false,
            skip_deps: false,
        }
    }
}

/// Executes a single task.
pub async fn execute_task(
    task: &Task,
    all_tasks: &[Task],
    config: &TaskExecutorConfig,
    semaphore: Arc<Semaphore>,
) -> anyhow::Result<bool> {
    if !task_needs_permit(task) {
        return Ok(false);
    }

    let _permit = if task_needs_permit(task) {
        Some(semaphore.acquire().await.unwrap())
    } else {
        None
    };

    let start = Instant::now();

    if config.dry_run {
        println!("[dry-run] Would run task: {}", task.name);
        return Ok(true);
    }

    // Resolve the working directory
    let cwd = if let Some(ref dir) = task.dir {
        let base = task
            .config_root
            .as_deref()
            .unwrap_or_else(|| Path::new("."));
        base.join(dir)
    } else {
        config
            .cd
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
    };

    // Execute each run entry
    for entry in &task.command {
        match entry {
            RunEntry::Script(script) => {
                let shell = task
                    .shell
                    .as_deref()
                    .or(config.shell.as_deref())
                    .unwrap_or("sh");
                let shell_flag = if cfg!(windows) { "/c" } else { "-c" };

                let mut cmd = tokio::process::Command::new(shell);
                cmd.arg(shell_flag);
                cmd.arg(script);
                cmd.current_dir(&cwd);

                // Set environment variables
                for (key, val) in &task.env {
                    cmd.env(key, val);
                }

                let status = cmd.status().await?;
                if !status.success() && !config.continue_on_error {
                    anyhow::bail!("Task '{}' failed with status: {}", task.name, status);
                }
            }
            RunEntry::SingleTask { task, args, env } => {
                // Find and execute the sub-task
                if let Some(sub_task) = all_tasks.iter().find(|t| t.name == *task) {
                    let mut sub = sub_task.clone();
                    sub.trailing_args = args.clone();
                    for (k, v) in env {
                        sub.env.insert(k.clone(), v.clone());
                    }
                    Box::pin(execute_task(&sub, all_tasks, config, semaphore.clone())).await?;
                }
            }
            RunEntry::TaskGroup { tasks } => {
                for task_name in tasks {
                    if let Some(sub_task) = all_tasks.iter().find(|t| t.name == *task_name) {
                        Box::pin(execute_task(sub_task, all_tasks, config, semaphore.clone()))
                            .await?;
                    }
                }
            }
        }
    }

    if config.timings {
        let elapsed = start.elapsed();
        println!("  {} completed in {:?}", task.name, elapsed);
    }

    Ok(true)
}

/// Display task results in the terminal.
pub fn display_task_start(task: &Task, output: TaskOutput) {
    match output {
        TaskOutput::Quiet | TaskOutput::Silent => {}
        _ => {
            let prefix = format!("[{}]", task.name);
            println!("{} {}", console::style(prefix).cyan().bold(), task.description);
        }
    }
}

pub fn display_task_finish(task: &Task, output: TaskOutput, duration: std::time::Duration) {
    match output {
        TaskOutput::Quiet | TaskOutput::Silent => {}
        _ => {
            let prefix = format!("[{}]", task.name);
            println!(
                "{} {} {}",
                console::style(prefix).green().bold(),
                "completed in",
                console::style(format!("{:?}", duration)).dim()
            );
        }
    }
}
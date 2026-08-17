// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

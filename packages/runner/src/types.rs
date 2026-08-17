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

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A task run entry — either a shell script, a single sub-task, or a task group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunEntry {
    Script(String),
    SingleTask {
        task: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: IndexMap<String, String>,
    },
    TaskGroup {
        tasks: Vec<String>,
    },
}

/// A dependency reference to another task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskDep {
    Simple(String),
    Detailed {
        task: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: IndexMap<String, String>,
        #[serde(default)]
        optional: bool,
    },
}

impl TaskDep {
    pub fn task_name(&self) -> &str {
        match self {
            TaskDep::Simple(s) => s,
            TaskDep::Detailed { task, .. } => task,
        }
    }
}

/// Tool value — either a simple version string or a map with version + options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskToolValue {
    String(String),
    Map(TaskToolValueMap),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskToolValueMap {
    pub version: String,
    #[serde(default)]
    pub opts: IndexMap<String, toml::Value>,
}

/// Task output style.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TaskOutput {
    Interleave,
    KeepOrder,
    Prefix,
    Replacing,
    Timed,
    Quiet,
    Silent,
}

/// Task outputs definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum TaskOutputs {
    Files(Vec<String>),
    NoFiles,
    #[default]
    Auto,
}

/// Task cache configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCacheConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub audit: bool,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub command_inputs: Vec<String>,
}

/// Task watch options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskWatchOptions {
    #[serde(default)]
    pub no_vcs_ignore: bool,
}

/// Task confirmation prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfirm {
    pub message: Option<String>,
}

/// Silent mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(untagged)]
pub enum Silent {
    #[default]
    Off,
    Bool(bool),
    Stdout,
    Stderr,
}

/// Full task definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Task {
    // Identity
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,

    // Config source
    #[serde(skip)]
    pub config_source: Option<PathBuf>,
    #[serde(skip)]
    pub config_root: Option<PathBuf>,

    // Confirmation
    #[serde(default)]
    pub confirm: Option<TaskConfirm>,

    // Dependencies
    #[serde(default)]
    #[serde(alias = "depends")]
    pub depends: Vec<TaskDep>,
    #[serde(default)]
    pub depends_post: Vec<TaskDep>,
    #[serde(default)]
    pub wait_for: Vec<TaskDep>,

    // Environment
    #[serde(default)]
    pub env: IndexMap<String, String>,
    #[serde(default)]
    pub vars: IndexMap<String, String>,

    // Execution
    #[serde(default)]
    pub dir: Option<String>,
    #[serde(default)]
    pub hide: bool,
    #[serde(default)]
    pub global: bool,
    #[serde(default)]
    pub raw: bool,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub watch: Option<TaskWatchOptions>,
    #[serde(default)]
    pub outputs: TaskOutputs,
    #[serde(default)]
    pub cache: Option<TaskCacheConfig>,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub quiet: bool,
    #[serde(default)]
    pub silent: Silent,
    #[serde(default)]
    pub output: Option<TaskOutput>,
    #[serde(default)]
    pub tools: IndexMap<String, TaskToolValue>,
    #[serde(default)]
    pub usage: String,
    #[serde(default)]
    pub timeout: Option<String>,

    // Run entries
    #[serde(default)]
    #[serde(alias = "run")]
    pub command: Vec<RunEntry>,
    #[serde(default)]
    pub run_windows: Vec<RunEntry>,

    // File reference
    #[serde(default)]
    pub file: Option<String>,

    // Sandbox
    #[serde(default)]
    pub deny_all: bool,
    #[serde(default)]
    pub deny_read: bool,
    #[serde(default)]
    pub deny_write: bool,
    #[serde(default)]
    pub deny_net: bool,
    #[serde(default)]
    pub deny_env: bool,
    #[serde(default)]
    pub allow_read: Vec<PathBuf>,
    #[serde(default)]
    pub allow_write: Vec<PathBuf>,
    #[serde(default)]
    pub allow_net: Vec<String>,
    #[serde(default)]
    pub allow_env: Vec<String>,
    #[serde(default)]
    pub pass_through_env: Vec<String>,

    // Template extension
    #[serde(default)]
    pub extends: Option<String>,

    // Trailing args
    #[serde(skip)]
    pub trailing_args: Vec<String>,
}

/// Task template definition (for [task_templates.*] section).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskTemplate {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub confirm: Option<TaskConfirm>,
    #[serde(default)]
    pub depends: Vec<TaskDep>,
    #[serde(default)]
    pub depends_post: Vec<TaskDep>,
    #[serde(default)]
    pub wait_for: Vec<TaskDep>,
    #[serde(default)]
    pub env: IndexMap<String, String>,
    #[serde(default)]
    pub vars: IndexMap<String, String>,
    #[serde(default)]
    pub dir: Option<String>,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub watch: Option<TaskWatchOptions>,
    #[serde(default)]
    pub outputs: TaskOutputs,
    #[serde(default)]
    pub cache: Option<TaskCacheConfig>,
    #[serde(default)]
    pub output: Option<TaskOutput>,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub silent: Option<Silent>,
    #[serde(default)]
    pub tools: IndexMap<String, TaskToolValue>,
    #[serde(default)]
    pub usage: String,
    #[serde(default)]
    pub timeout: Option<String>,
    #[serde(default)]
    pub command: Vec<RunEntry>,
    #[serde(default)]
    pub run_windows: Vec<RunEntry>,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub deny_all: bool,
    #[serde(default)]
    pub deny_read: bool,
    #[serde(default)]
    pub deny_write: bool,
    #[serde(default)]
    pub deny_net: bool,
    #[serde(default)]
    pub deny_env: bool,
    #[serde(default)]
    pub allow_read: Vec<PathBuf>,
    #[serde(default)]
    pub allow_write: Vec<PathBuf>,
    #[serde(default)]
    pub allow_net: Vec<String>,
    #[serde(default)]
    pub allow_env: Vec<String>,
    #[serde(default)]
    pub pass_through_env: Vec<String>,
}

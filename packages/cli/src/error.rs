// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use montrs_core::AgentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Task execution failed: {0}")]
    Task(String),
    #[error("Build failed: {0}")]
    Build(String),
}

impl AgentError for CliError {
    fn error_code(&self) -> &'static str {
        match self {
            CliError::Config(_) => "CLI_CONFIG",
            CliError::Io(_) => "CLI_IO",
            CliError::Task(_) => "CLI_TASK",
            CliError::Build(_) => "CLI_BUILD",
        }
    }

    fn explanation(&self) -> String {
        match self {
            CliError::Config(e) => format!(
                "Failed to load or parse the MontRS configuration: {}.",
                e
            ),
            CliError::Io(e) => {
                format!("An I/O error occurred during CLI operation: {}.", e)
            }
            CliError::Task(e) => {
                format!("A custom task failed to execute: {}.", e)
            }
            CliError::Build(e) => {
                format!("The project build process failed: {}.", e)
            }
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            CliError::Config(_) => vec![
                "Check montrs.toml for syntax errors.".to_string(),
                "Ensure all required configuration fields are present."
                    .to_string(),
            ],
            CliError::Io(_) => {
                vec!["Verify file permissions and paths.".to_string()]
            }
            CliError::Task(e) => vec![format!("Debug the task logic: {}.", e)],
            CliError::Build(_) => vec![
                "Check the compiler output for detailed error messages."
                    .to_string(),
                "Ensure all dependencies are correctly specified in \
                 Cargo.toml."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "cli"
    }

    fn documentation_refs(&self) -> Vec<String> {
        match self {
            CliError::Config(_) => {
                vec!["packages/cli/docs/invariants".to_string()]
            }
            CliError::Io(_) => vec!["packages/cli/docs/invariants".to_string()],
            CliError::Task(_) => {
                vec!["packages/cli/docs/invariants".to_string()]
            }
            CliError::Build(_) => {
                vec!["packages/cli/docs/invariants".to_string()]
            }
        }
    }
}

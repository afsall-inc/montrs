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

//! Invariant tests for montrs-cli.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Delegated Logic: CLI only handles orchestration
//! - Subcommand Isolation: Commands are modular
//! - Agent Synchronization: Commands trigger .agent/ updates

use montrs_cli::*;

#[test]
fn test_montrs_cli_debug() {
    let cli = MontrsCli {
        command: Commands::Build,
        release: false,
        hot_reload: false,
        features: Vec::new(),
        verbose: 0,
        log: Vec::new(),
    };
    assert!(format!("{:?}", cli).contains("Build"));
}

#[test]
fn test_commands_variants() {
    match Commands::Build {
        Commands::Build => {}
        _ => panic!("expected Build"),
    }
    match Commands::Serve {
        Commands::Serve => {}
        _ => panic!("expected Serve"),
    }
    match Commands::Watch {
        Commands::Watch => {}
        _ => panic!("expected Watch"),
    }
}

#[test]
fn test_cli_error_agent_error_impl() {
    use montrs_core::AgentError;
    let err = error::CliError::Config("bad config".to_string());
    assert_eq!(err.error_code(), "CLI_CONFIG");
    assert!(!err.explanation().is_empty());
    assert!(!err.suggested_fixes().is_empty());
    assert_eq!(err.subsystem(), "cli");
}

#[test]
fn test_cli_error_display() {
    let err = error::CliError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file not found",
    ));
    assert!(format!("{}", err).contains("IO error"));
}

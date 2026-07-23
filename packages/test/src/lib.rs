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

//! # montrs-test
//!
//! // @agent-tool: name="test_run" desc="Runs unit, integration, and end-to-end tests for the MontRS project."
//!
//! Utilities for deterministic, robust testing of MontRS applications.
//!
//! This crate provides the foundational infrastructure needed to write unit, integration,
//! and end-to-end (E2E) tests. It allows you to:
//!
//! - **Mock Environment Variables**: Use `TestEnv` to simulate different runtime configurations.
//! - **Manage Test Lifecycles**: Use `Fixture` and `run_fixture_test` for setup/teardown logic.
//! - **Run E2E Tests**: Use `MontrsDriver` (via the `e2e` feature) to control browsers with Playwright.
//! - **Simulate Application Runtime**: Use `TestRuntime` to execute application logic in-process.
//!
//! The E2E capabilities are integrated with `TestRuntime`, allowing you to easily spin up
//! browser tests alongside your integration tests.
//!
//! ## Feature Flags
//!
//! - `e2e`: Enables End-to-End testing capabilities using `playwright-rs`.
//!
//! ## Example
//!
//! ```rust
//! use montrs_core::env::EnvConfig;
//! use montrs_test::TestEnv;
//!
//! let env = TestEnv::new();
//! env.set("DATABASE_URL", "sqlite::memory:");
//! assert_eq!(env.get_var("DATABASE_URL").unwrap(), "sqlite::memory:");
//! ```

pub mod integration;
pub mod unit;

#[cfg(feature = "e2e")]
pub mod e2e;

pub use integration::{Fixture, TestEnv, TestRuntime, run_fixture_test};
use montrs_core::AgentError;
use thiserror::Error;
pub use unit::{Mock, Spy, expect, simple_bench};

/// Errors that can occur during testing.
#[derive(Error, Debug)]
pub enum TestError {
    #[error("Fixture setup failed: {0}")]
    Setup(String),
    #[error("Fixture teardown failed: {0}")]
    Teardown(String),
    #[error("E2E driver error: {0}")]
    E2e(String),
    #[error("Expectation failed: {0}")]
    Expectation(String),
    #[error("IO error during testing: {0}")]
    Io(#[from] std::io::Error),
}

impl AgentError for TestError {
    fn error_code(&self) -> &'static str {
        match self {
            TestError::Setup(_) => "TEST_SETUP",
            TestError::Teardown(_) => "TEST_TEARDOWN",
            TestError::E2e(_) => "TEST_E2E",
            TestError::Expectation(_) => "TEST_EXPECTATION",
            TestError::Io(_) => "TEST_IO",
        }
    }

    fn explanation(&self) -> String {
        match self {
            TestError::Setup(e) => {
                format!("A test fixture failed to set up: {}.", e)
            }
            TestError::Teardown(e) => {
                format!("A test fixture failed to tear down: {}.", e)
            }
            TestError::E2e(e) => format!(
                "An error occurred in the E2E driver (Playwright): {}.",
                e
            ),
            TestError::Expectation(e) => {
                format!("A test expectation was not met: {}.", e)
            }
            TestError::Io(e) => format!(
                "An I/O error occurred during the test execution: {}.",
                e
            ),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            TestError::Setup(_) => vec![
                "Check the setup method of your fixture for errors."
                    .to_string(),
                "Ensure that any required external services (e.g., databases) \
                 are available."
                    .to_string(),
            ],
            TestError::Teardown(_) => vec![
                "Check the teardown method of your fixture for errors."
                    .to_string(),
                "Ensure that resources are being properly released."
                    .to_string(),
            ],
            TestError::E2e(_) => vec![
                "Verify that Playwright is correctly installed and configured."
                    .to_string(),
                "Check if the browser can be launched and the target URL is \
                 accessible."
                    .to_string(),
            ],
            TestError::Expectation(_) => vec![
                "Review the test logic and the actual vs. expected values."
                    .to_string(),
                "Debug the code being tested to find the cause of the \
                 discrepancy."
                    .to_string(),
            ],
            TestError::Io(_) => vec![
                "Check if the file system is accessible and you have the \
                 necessary permissions."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "test"
    }
}


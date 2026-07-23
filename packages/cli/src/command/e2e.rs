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

//! E2E test command.
//!
//! This plate runs the full end-to-end testing pipeline. It coordinates:
//! 1. Building the application.
//! 2. Starting the backend server.
//! 3. Running the E2E test suite against the running server.
//!
//! It delegates the heavy lifting to `cargo-leptos` but ensures the
//! MontRS configuration is correctly mapped.

use crate::{config::MontrsConfig, utils::run_cargo_leptos};

/// Executes the E2E tests.
pub async fn run(
    headless: bool,
    keep_alive: bool,
    browser: Option<String>,
) -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;

    // Determine final configuration (CLI > Config > Default)
    let final_headless = headless || config.e2e.headless.unwrap_or(false);
    let final_browser = browser
        .or(config.e2e.browser.clone())
        .unwrap_or_else(|| "chromium".to_string());

    // Set environment variables for runtime configuration
    unsafe {
        std::env::set_var("MONTRS_E2E_HEADLESS", final_headless.to_string());
        if keep_alive {
            std::env::set_var("MONTRS_E2E_KEEP_ALIVE", "true");
        }
        std::env::set_var("MONTRS_E2E_BROWSER", final_browser);

        if let Some(url) = &config.e2e.base_url {
            std::env::set_var("MONTRS_SITE_URL", url);
        }

        // Default defaults for MontRS structure
        if std::env::var("LEPTOS_END2END_CMD").is_err() {
            std::env::set_var("LEPTOS_END2END_CMD", "cargo test --package e2e");
        }
        if std::env::var("LEPTOS_END2END_DIR").is_err() {
            std::env::set_var("LEPTOS_END2END_DIR", "e2e");
        }
    }

    // We use "end-to-end" command of cargo-leptos.
    // This command requires configuration in Cargo.toml (or env vars) for "end2end-cmd"
    // and "end2end-dir".

    // Ensure we pass necessary flags via args if supported, or rely on env vars set above.

    run_cargo_leptos("end-to-end", &[], &config).await
}

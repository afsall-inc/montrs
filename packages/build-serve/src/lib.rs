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

//! montrs-build-serve: Dev server for MontRS projects.
//!
//! Serves the site root directory and optionally spawns the SSR server.
//! Extracted from `montrs-build` to separate the HTTP serving concern
//! from the build pipeline.

use anyhow::Result;
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::info;

/// Configuration for the dev server.
#[derive(Debug, Clone)]
pub struct ServeConfig {
    /// The address to bind to (e.g., "0.0.0.0:3000").
    pub addr: String,
    /// The root directory to serve files from.
    pub site_root: PathBuf,
    /// The WASM package directory relative to site_root.
    pub pkg_dir: PathBuf,
}

/// Start the static file dev server.
///
/// Serves files from `site_root` and logs the address. This is a
/// lightweight static file server — the SSR server binary is spawned
/// separately by the CLI.
pub async fn serve_static(config: ServeConfig) -> Result<()> {
    let app = Router::new().fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev server listening on {}", config.addr);
    info!("Serving from {}", config.site_root.display());

    axum::serve(listener, app).await?;
    Ok(())
}

/// Start the dev server with a callback for when the server is ready.
pub async fn serve_with_callback<F>(
    config: ServeConfig,
    on_ready: F,
) -> Result<()>
where
    F: FnOnce(),
{
    let app = Router::new().fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev server listening on {}", config.addr);
    on_ready();

    axum::serve(listener, app).await?;
    Ok(())
}

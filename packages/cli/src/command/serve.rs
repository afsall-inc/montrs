// Ø¨ÙØ³Ù’Ù…Ù Ø§Ù„Ù„ÙŽÙ‘Ù‡Ù Ø§Ù„Ø±ÙŽÙ‘Ø­Ù’Ù…ÙŽÙ†Ù Ø§Ù„Ø±ÙŽÙ‘Ø­ÙÙŠÙ…
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

use montrs_build::{BuildPipeline, Pipeline, reload::LiveReload};
use std::path::Path;
use tokio::process::Command as TokioCommand;

pub async fn run() -> anyhow::Result<()> {
    let mut pipeline = match Pipeline::from_root(Path::new(".")) {
        Ok(p) => p,
        Err(e) => {
            anyhow::bail!(
                "Could not find montrs.toml in the current directory. Are you \
                 in a MontRS project? Error: {e}"
            );
        }
    };
    pipeline.release |= crate::config::current_release();

    crate::command::resolve_pipeline_bins(&mut pipeline);
    pipeline.build_all()?;

    let addr = pipeline.meta.serve.site_addr.clone();
    let site_root = pipeline.site_root.to_string_lossy().to_string();
    // pkg_dir is a URL path relative to the site root (e.g. "pkg"); passing
    // the absolute filesystem path here leaks `\\?\C:\...` into the hydration
    // bootstrap's `import()` specifier and breaks WASM loading.
    let pkg_dir = pipeline
        .meta
        .serve
        .site_pkg_dir
        .trim_end_matches('/')
        .to_string();

    let bin = pipeline.server_bin_path();

    if !bin.exists() {
        anyhow::bail!(
            "SSR server binary not found at {}. Build may have failed.",
            bin.display()
        );
    }

    println!("Starting SSR server at {addr}");
    println!("Site root: {site_root}");
    println!("PKG dir: {pkg_dir}");

    // Live-reload: the SSR shell's hydration scripts open a WebSocket here
    // and reload the tab whenever we broadcast after a successful rebuild.
    let reload_port = pipeline.meta.serve.reload_port;
    let reload = match LiveReload::start(reload_port).await {
        Ok(r) => {
            println!("Live reload listening on ws://0.0.0.0:{reload_port}");
            Some(r)
        }
        Err(e) => {
            eprintln!(
                "Live reload unavailable ({e}); page won't auto-refresh."
            );
            None
        }
    };

    let mut server_child = TokioCommand::new(&bin)
        .env("MONTRS_SITE_ROOT", &site_root)
        .env("MONTRS_SITE_PKG_DIR", &pkg_dir)
        .env("MONTRS_SITE_ADDR", &addr)
        .env("MONTRS_RELOAD_PORT", reload_port.to_string())
        .env(
            "MONTRS_OUTPUT_NAME",
            pipeline
                .meta
                .serve
                .output_name
                .as_deref()
                .unwrap_or("website"),
        )
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;

    // Watch for source changes: rebuild, then tell the browser to reload.
    let reload_for_watch = reload.clone();
    let _watcher = tokio::task::spawn_blocking(move || {
        let _ = montrs_build::watch_directory(Path::new("."), move || {
            println!("Change detected — rebuilding...");
            match pipeline.build_all() {
                Ok(_) => {
                    println!("Rebuild complete.");
                    if let Some(r) = &reload_for_watch {
                        r.notify();
                    }
                }
                Err(e) => eprintln!("Build error: {e}"),
            }
        });
    });

    let status = server_child.wait().await?;
    if !status.success() {
        anyhow::bail!("SSR server exited with error code");
    }

    Ok(())
}

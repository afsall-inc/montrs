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

//! `montrs serve` / `montrs watch` supervisor.
//!
//! The dev loop never bails on a build error: it broadcasts typed events to the
//! browser overlay, keeps the last-good SSR server running, and restarts it
//! after each successful rebuild. If the first build fails, a lightweight
//! fallback page is served on the site address so the overlay and live-reload
//! socket stay reachable.

use montrs_build::{BuildPipeline, Pipeline, reload::LiveReload};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
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

    let addr = pipeline.meta.serve.site_addr.clone();
    let site_root = pipeline.site_root.to_string_lossy().to_string();
    let pkg_dir = pipeline
        .meta
        .serve
        .site_pkg_dir
        .trim_end_matches('/')
        .to_string();
    let output_name = pipeline
        .meta
        .serve
        .output_name
        .as_deref()
        .unwrap_or("website")
        .to_string();
    let reload_port = pipeline.meta.serve.reload_port;
    let bin = pipeline.server_bin_path();

    println!("Serving on http://{addr}");
    println!("Site root: {site_root}");
    println!("PKG dir: {pkg_dir}");

    // Live-reload first, so even a failed first build can report to the browser.
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

    // Watch channel: the blocking file watcher sends a signal here.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
    let pipeline_arc = Arc::new(pipeline);
    let _watcher = tokio::task::spawn_blocking({
        let tx = tx.clone();
        move || {
            let _ = montrs_build::watch_directory(Path::new("."), move || {
                let _ = tx.blocking_send(());
            });
        }
    });

    // Initial build (non-fatal). A failure should not kill the dev loop.
    let mut have_server = match pipeline_arc.build_all() {
        Ok(()) => {
            println!("Initial build complete.");
            true
        }
        Err(e) => {
            eprintln!("Initial build failed: {e}");
            if let Some(r) = &reload {
                report_build_error(r, &e);
            }
            false
        }
    };

    // If no SSR binary exists yet, serve the fallback page on the site address.
    let mut fallback: Option<tokio::task::JoinHandle<()>> = None;
    let mut fallback_shutdown: Option<tokio::sync::oneshot::Sender<()>> = None;
    if !have_server {
        let (s, r) = tokio::sync::oneshot::channel::<()>();
        fallback_shutdown = Some(s);
        let cfg = montrs_build_serve::ServeConfig {
            addr: addr.clone(),
            site_root: site_root.clone().into(),
            pkg_dir: pkg_dir.clone().into(),
        };
        fallback = Some(tokio::spawn(async move {
            let shutdown = async move { let _ = r.await; };
            if let Err(e) = montrs_build_serve::serve_fallback_with_shutdown(
                cfg, reload_port, shutdown,
            )
            .await
            {
                eprintln!("Fallback server error: {e}");
            }
        }));
        println!("Initial build failed — serving the dev fallback page.");
    }

    // Supervisor loop: rebuild on change, supervise the SSR child, and never
    // bail on build errors.
    let mut child: Option<tokio::process::Child> = None;
    let mut backoff = Duration::from_millis(250);

    loop {
        // Ensure the SSR server is running.
        if have_server && child.is_none() {
            // Hand the site address back to the real server: stop the fallback
            // first, wait for the socket to be released, then spawn the SSR
            // child.
            if let Some(sh) = fallback_shutdown.take() {
                let _ = sh.send(());
                if let Some(h) = fallback.take() {
                    let _ = tokio::time::timeout(Duration::from_secs(3), h).await;
                }
            }
            match spawn_server(
                &bin,
                &addr,
                &site_root,
                &pkg_dir,
                &output_name,
                reload_port,
            ) {
                Ok(c) => {
                    child = Some(c);
                    println!("SSR server started.");
                }
                Err(e) => {
                    if let Some(r) = &reload {
                        r.server_error(&e);
                    }
                    eprintln!("Could not start SSR server: {e}");
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(5));
                    continue;
                }
            }
            backoff = Duration::from_millis(250);
        }

        tokio::select! {
            Some(()) = rx.recv() => {
                println!("Change detected — rebuilding...");
                if let Some(r) = &reload {
                    r.building();
                }
                match pipeline_arc.build_all() {
                    Ok(()) => {
                        println!("Rebuild complete.");
                        if let Some(r) = &reload {
                            r.build_ok();
                        }
                        have_server = true;
                        // Restart the SSR child so it picks up the new code.
                        if let Some(mut c) = child.take() {
                            let _ = c.kill().await;
                            let _ = c.wait().await;
                        }
                    }
                    Err(e) => {
                        eprintln!("Build error: {e}");
                        if let Some(r) = &reload {
                            report_build_error(r, &e);
                        }
                        // Keep the last-good server running.
                    }
                }
            }
            status = async { child.as_mut().unwrap().wait().await }, if child.is_some() => {
                child = None;
                if have_server {
                    if let Some(r) = &reload {
                        r.server_error(&format!("SSR server exited ({status:?})"));
                    }
                    eprintln!("SSR server exited ({status:?}); restarting...");
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(5));
                }
            }
        }
    }
}

fn spawn_server(
    bin: &Path,
    addr: &str,
    site_root: &str,
    pkg_dir: &str,
    output_name: &str,
    reload_port: u16,
) -> Result<tokio::process::Child, String> {
    if !bin.exists() {
        return Err(format!(
            "SSR server binary not found at {}. Build may have failed.",
            bin.display()
        ));
    }
    TokioCommand::new(bin)
        .env("MONTRS_SITE_ROOT", site_root)
        .env("MONTRS_SITE_PKG_DIR", pkg_dir)
        .env("MONTRS_SITE_ADDR", addr)
        .env("MONTRS_RELOAD_PORT", reload_port.to_string())
        .env("MONTRS_OUTPUT_NAME", output_name)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| format!("failed to spawn SSR server: {e}"))
}

fn report_build_error(reload: &LiveReload, err: &anyhow::Error) {
    let msg = err.to_string();
    let (file, line, column, frame) = parse_compiler_error(&msg);
    reload.build_error(&msg, file.as_deref(), line, column, frame.as_deref());
}

fn parse_compiler_error(msg: &str) -> (Option<String>, Option<u32>, Option<u32>, Option<String>) {
    let mut file = None;
    let mut line = None;
    let mut column = None;
    let mut frame = None;

    if let Some(start) = msg.find("error[") {
        let end = msg[start + 6..]
            .find("error[")
            .map(|i| start + 6 + i)
            .unwrap_or(msg.len());
        let block = &msg[start..end];
        for l in block.lines() {
            if let Some(rest) = l.trim_start().strip_prefix("--> ") {
                if let Some((f, rest)) = rest.split_once(':') {
                    file = Some(f.to_string());
                    if let Some((ln, rest)) = rest.split_once(':') {
                        line = ln.parse().ok();
                        column = rest.split_once(':')
                            .and_then(|(c, _)| c.parse().ok())
                            .or_else(|| rest.trim().parse().ok());
                    }
                }
                break;
            }
        }
        let mut lines: Vec<&str> = block.lines().collect();
        if !lines.is_empty() {
            lines.remove(0);
        }
        if !lines.is_empty() && lines[0].trim_start().starts_with("-->") {
            lines.remove(0);
        }
        let text = lines.join("\n").trim().to_string();
        if !text.is_empty() {
            frame = Some(text);
        }
    }

    (file, line, column, frame)
}

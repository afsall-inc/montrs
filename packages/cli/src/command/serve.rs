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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::process::Command as TokioCommand;
use tokio::task::JoinHandle;

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

    // The dev server always builds in the dev profile. This keeps
    // `debug_assertions` on so the SSR HTML carries the Leptos hot-reload
    // markers, and makes incremental rebuilds far faster.
    pipeline.release = false;
    // Build the WASM client with the matching `hot` profile too, so both sides
    // emit the same hot-reload markers (otherwise `tachys` hydration panics and
    // nothing is interactive).
    pipeline.hot_reload = true;

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
    let mut reload_port = pipeline.meta.serve.reload_port;
    let bin = pipeline.server_bin_path();

    // Watch the app's source trees plus the workspace `packages/` tree (when
    // present) so edits to framework crates — not just the app — trigger a
    // rebuild. Watching the app root directly would drag in its `target/site`
    // output and flood the watcher during every build, so watch only sources.
    let mut watch_roots: Vec<PathBuf> = Vec::new();
    for candidate in ["app", "src", "style", "assets"] {
        let dir = pipeline.project_root.join(candidate);
        if dir.exists() {
            watch_roots.push(dir);
        }
    }
    for manifest in ["Cargo.toml", "montrs.toml"] {
        let file = pipeline.project_root.join(manifest);
        if file.exists() {
            watch_roots.push(file);
        }
    }
    if let Some(ws_root) = pipeline.workspace_target_dir.parent() {
        let packages = ws_root.join("packages");
        if packages.exists() {
            watch_roots.push(packages);
        }
    }
    if watch_roots.is_empty() {
        watch_roots.push(pipeline.project_root.clone());
    }

    // Source roots scanned for `view!` macros so markup edits can be patched
    // into the browser without a rebuild. The app plus the UI component crate
    // are where view macros live.
    let mut view_roots: Vec<PathBuf> = Vec::new();
    for candidate in ["app", "src"] {
        let dir = pipeline.project_root.join(candidate);
        if dir.exists() {
            view_roots.push(dir);
        }
    }
    if let Some(ws_root) = pipeline.workspace_target_dir.parent() {
        let ui = ws_root.join("packages").join("ui");
        if ui.exists() {
            view_roots.push(ui);
        }
    }
    // cargo derives the `view!` stable ids from workspace-relative paths.
    let workspace_root =
        pipeline.workspace_target_dir.parent().map(|p| p.to_path_buf());

    // Experimental (Stage 1 foundation): capture each workspace crate's rustc
    // invocation so a later hot-patch pass can replay only changed crates.
    // Enabled with `MONTRS_HOTPATCH=1`.
    let hotpatch_enabled = std::env::var_os("MONTRS_HOTPATCH").is_some();
    let hotpatch_dir = pipeline.workspace_target_dir.join("montrs-hotpatch");
    let hotpatch_workspace_target = pipeline.workspace_target_dir.clone();
    if hotpatch_enabled {
        match resolve_wrapper("rustc-wrapper") {
            Some(wrapper) => {
                let _ = std::fs::remove_dir_all(&hotpatch_dir);
                // SAFETY: set once, before any build starts; the cargo child
                // processes inherit this environment.
                unsafe {
                    std::env::set_var("RUSTC_WORKSPACE_WRAPPER", &wrapper);
                    std::env::set_var("MONTRS_HOTPATCH_DIR", &hotpatch_dir);
                }
                println!(
                    "Hot-patch capture enabled (rustc wrapper: {}).",
                    wrapper.display()
                );
                println!("  capture dir: {}", hotpatch_dir.display());

                if let Some(link_wrapper) = resolve_wrapper("link-wrapper") {
                    match (discover_real_linker(), host_triple()) {
                        (Some(real), Some(host)) => {
                            let key = format!(
                                "CARGO_TARGET_{}_LINKER",
                                host.replace('-', "_").to_uppercase()
                            );
                            unsafe {
                                std::env::set_var(
                                    "MONTRS_REAL_LINKER",
                                    &real,
                                );
                                std::env::set_var(key, &link_wrapper);
                            }
                            let flavor =
                                montrs_dev_hotpatch::flavor_from_triple(&host);
                            // The fat (hot-patchable) relink happens inside the
                            // linker shim during the tip link, while rustc's
                            // temporary objects still exist.
                            let flavor_env = match flavor {
                                montrs_dev_hotpatch::LinkerFlavor::Msvc => {
                                    "msvc"
                                }
                                montrs_dev_hotpatch::LinkerFlavor::Gnu => {
                                    "gnu"
                                }
                                montrs_dev_hotpatch::LinkerFlavor::Other => {
                                    "other"
                                }
                            };
                            unsafe {
                                std::env::set_var(
                                    "MONTRS_HOTPATCH_FATLINK",
                                    "1",
                                );
                                std::env::set_var(
                                    "MONTRS_HOTPATCH_TIP_OUT",
                                    &bin,
                                );
                                std::env::set_var(
                                    "MONTRS_HOTPATCH_WORKSPACE",
                                    &hotpatch_workspace_target,
                                );
                                std::env::set_var(
                                    "MONTRS_HOTPATCH_FLAVOR",
                                    flavor_env,
                                );
                            }
                            println!(
                                "  link capture enabled (real linker: {}).",
                                real.display()
                            );
                        }
                        _ => eprintln!(
                            "  link capture skipped: could not determine the \
                             host triple or real linker."
                        ),
                    }
                } else {
                    eprintln!(
                        "  link capture skipped: montrs-link-wrapper not found."
                    );
                }
            }
            None => {
                eprintln!(
                    "MONTRS_HOTPATCH is set but montrs-rustc-wrapper was not \
                     found. Install it with:\n  cargo install --path \
                     packages/dev-hotpatch --bin montrs-rustc-wrapper --bin \
                     montrs-link-wrapper"
                );
            }
        }
    }

    println!("Serving on http://{addr}");
    println!("Site root: {site_root}");
    println!("PKG dir: {pkg_dir}");

    // Live-reload first, so even a failed first build can report to the browser.
    // If the configured port is taken, the server binds another and we adopt the
    // actual port everywhere below (SSR child env + injected meta tag).
    let reload = match LiveReload::start(reload_port).await {
        Ok((r, port)) => {
            if port != reload_port {
                println!(
                    "Live reload port {reload_port} was busy — using {port}."
                );
            }
            reload_port = port;
            println!("Live reload listening on ws://0.0.0.0:{port}");
            Some(r)
        }
        Err(e) => {
            eprintln!(
                "Live reload unavailable ({e}); page won't auto-refresh."
            );
            None
        }
    };

    // Hot-patch socket: clients report their runtime base address and receive
    // jump tables to apply. Started alongside capture.
    let hotpatch_server = if hotpatch_enabled {
        match montrs_dev_hotpatch::server::HotPatchServer::start(
            reload_port.saturating_add(1),
        )
        .await
        {
            Ok((server, port)) => {
                // The SSR child bridges `/_dioxus` to this address.
                unsafe {
                    std::env::set_var(
                        "MONTRS_HOTPATCH_ADDR",
                        format!("127.0.0.1:{port}"),
                    );
                }
                println!("Hot-patch socket on ws://0.0.0.0:{port}");
                Some(server)
            }
            Err(e) => {
                eprintln!("Hot-patch socket unavailable ({e}).");
                None
            }
        }
    } else {
        None
    };

    // Watch channel: the blocking file watcher signals a rebuild here. View
    // and CSS edits are handled entirely inside the watcher thread (instant
    // patches, no cargo) and never reach this channel.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
    let pipeline_arc = Arc::new(pipeline);
    let _watcher = tokio::task::spawn_blocking({
        let tx = tx.clone();
        let reload = reload.clone();
        let pipeline_for_watch = pipeline_arc.clone();
        move || {
            // Baseline of every `view!` macro in the app, used to diff edits.
            let mut patcher = montrs_hot_reload::ViewPatcher::new(
                &view_roots,
                workspace_root,
            );
            let _ = montrs_build::watch_paths(
                &watch_roots,
                move |changed: &[PathBuf]| {
                    let mut rebuild = false;
                    let mut css_changed = false;
                    for path in changed {
                        match path.extension().and_then(|e| e.to_str()) {
                            Some("rs") => {
                                if let Some(patches) = patcher.patch(path)
                                    && let Ok(json) =
                                        serde_json::to_string(&patches)
                                    && let Some(r) = &reload
                                {
                                    r.view(json);
                                }
                                if !patcher.is_view_only(path) {
                                    rebuild = true;
                                }
                            }
                            Some("css") => css_changed = true,
                            _ => rebuild = true,
                        }
                    }
                    if css_changed {
                        if let Err(e) = pipeline_for_watch.process_tailwind() {
                            eprintln!("Tailwind error: {e}");
                        }
                        if let Some(r) = &reload {
                            r.notify_css("main.css");
                        }
                    }
                    if rebuild {
                        let _ = tx.blocking_send(());
                    }
                },
            );
        }
    });

    // Initial build (non-fatal, off the async worker). A failure should not
    // kill the dev loop.
    let mut have_server = match build_blocking(pipeline_arc.clone()).await {
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

    if hotpatch_enabled {
        log_capture(&hotpatch_dir);
    }

    // If no SSR binary exists yet, serve the fallback page on the site address.
    // The same page takes over the address while each later rebuild runs.
    let mut fallback: Option<JoinHandle<()>> = None;
    let mut fallback_shutdown: Option<tokio::sync::oneshot::Sender<()>> = None;
    if !have_server {
        let (h, s) =
            spawn_fallback(&addr, &site_root, &pkg_dir, reload_port);
        fallback = Some(h);
        fallback_shutdown = Some(s);
        println!("Initial build failed — serving the dev fallback page.");
    }

    // Supervisor loop: rebuild on change, supervise the SSR child, and never
    // bail on build errors.
    let mut child: Option<tokio::process::Child> = None;
    let mut backoff = Duration::from_millis(250);
    // Set after a successful rebuild; the "reload" is emitted only once the
    // restarted SSR child is actually listening (never before).
    let mut pending_reload = false;

    loop {
        // Ensure the SSR server is running — but never while the fallback page
        // owns the socket (i.e. during a rebuild or after a failed build).
        if have_server && child.is_none() && fallback.is_none() {
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
                    backoff = Duration::from_millis(250);
                    // Now that the new server is up, tell every tab to reload.
                    if pending_reload {
                        pending_reload = false;
                        if let Some(r) = &reload {
                            r.build_ok();
                            // Leptos-compatible reload frame.
                            r.notify();
                        }
                    }
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
        }

        tokio::select! {
            Some(()) = rx.recv() => {
                println!("Change detected — rebuilding...");
                if let Some(r) = &reload {
                    r.building();
                }

                // Stop the SSR child *before* building: cargo cannot replace a
                // running executable on Windows, and doing so would otherwise
                // abort the whole build before the WASM/CSS steps ran.
                if let Some(mut c) = child.take() {
                    let _ = c.kill().await;
                    let _ = c.wait().await;
                }

                // Serve the "compiling…" fallback on the site address while we
                // build, so navigating to the app keeps working (and reports
                // any build error). It auto-reloads on `build-ok`.
                if fallback.is_none() {
                    let (h, s) = spawn_fallback(
                        &addr, &site_root, &pkg_dir, reload_port,
                    );
                    fallback = Some(h);
                    fallback_shutdown = Some(s);
                }

                match build_blocking(pipeline_arc.clone()).await {
                    Ok(()) => {
                        println!("Rebuild complete.");
                        if hotpatch_enabled {
                            log_capture(&hotpatch_dir);
                            if std::env::var_os("MONTRS_HOTPATCH_PATCH")
                                .is_some()
                            {
                                try_build_patch(
                                    &bin,
                                    &hotpatch_workspace_target,
                                    &hotpatch_dir,
                                    hotpatch_server.as_ref(),
                                );
                            }
                        }
                        // Hand the address back to the real server on the next
                        // loop iteration. The reload is deferred until the new
                        // child is listening (see the spawn branch above).
                        stop_fallback(&mut fallback, &mut fallback_shutdown).await;
                        have_server = true;
                        backoff = Duration::from_millis(250);
                        pending_reload = true;
                    }
                    Err(e) => {
                        eprintln!("Build error: {e}");
                        if let Some(r) = &reload {
                            report_build_error(r, &e);
                        }
                        // Keep showing the fallback page (it renders the error)
                        // and keep the last-good WASM on disk for the retry.
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

/// Run the (blocking) build pipeline off the async worker threads so the
/// live-reload socket keeps accepting connections while cargo runs.
async fn build_blocking(
    pipeline: Arc<Pipeline>,
) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || pipeline.build_all())
        .await
        .map_err(|e| anyhow::anyhow!("build task panicked: {e}"))?
}

/// Spawn the fallback ("compiling…") server on the site address. Returns its
/// task handle and a shutdown signal.
fn spawn_fallback(
    addr: &str,
    site_root: &str,
    pkg_dir: &str,
    reload_port: u16,
) -> (JoinHandle<()>, tokio::sync::oneshot::Sender<()>) {
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let cfg = montrs_build_serve::ServeConfig {
        addr: addr.to_string(),
        site_root: site_root.into(),
        pkg_dir: pkg_dir.into(),
    };
    let handle = tokio::spawn(async move {
        let shutdown = async move {
            let _ = shutdown_rx.await;
        };
        if let Err(e) =
            montrs_build_serve::serve_fallback_with_shutdown(cfg, reload_port, shutdown)
                .await
        {
            eprintln!("Fallback server error: {e}");
        }
    });
    (handle, shutdown_tx)
}

/// Stop the fallback server and wait (briefly) for the socket to be released
/// so the SSR child can bind the same address.
async fn stop_fallback(
    fallback: &mut Option<JoinHandle<()>>,
    shutdown: &mut Option<tokio::sync::oneshot::Sender<()>>,
) {
    if let Some(s) = shutdown.take() {
        let _ = s.send(());
    }
    if let Some(h) = fallback.take() {
        let _ = tokio::time::timeout(Duration::from_secs(3), h).await;
    }
}

/// Log how many rustc invocations the hot-patch wrapper captured.
fn log_capture(dir: &Path) {
    let count = montrs_dev_hotpatch::read_rustc_invocations().len();
    println!(
        "Hot-patch capture: {count} rustc invocations recorded ({}).",
        dir.display()
    );
}

/// Attempt to build a patch from the freshly captured tip objects.
///
/// Opt-in via `MONTRS_HOTPATCH_PATCH=1`. The client's runtime base address
/// comes from a connected client (`ClientMsg::AslrReference`), falling back to
/// `MONTRS_HOTPATCH_ASLR` (hex); the built table is broadcast on the hot-patch
/// socket.
fn try_build_patch(
    bin: &Path,
    workspace_target_dir: &Path,
    hotpatch_dir: &Path,
    server: Option<&montrs_dev_hotpatch::server::HotPatchServer>,
) {
    let Ok(linker) = std::env::var("MONTRS_REAL_LINKER") else {
        eprintln!("Hot-patch: real linker unknown; skipping patch build.");
        return;
    };
    let flavor = match std::env::var("MONTRS_HOTPATCH_FLAVOR").as_deref() {
        Ok("msvc") => montrs_dev_hotpatch::LinkerFlavor::Msvc,
        Ok("gnu") => montrs_dev_hotpatch::LinkerFlavor::Gnu,
        _ => montrs_dev_hotpatch::LinkerFlavor::Other,
    };

    let report = server.and_then(|s| s.latest_aslr());
    let aslr_reference = report
        .map(|r| r.aslr_reference)
        .or_else(|| {
            std::env::var("MONTRS_HOTPATCH_ASLR").ok().and_then(|s| {
                u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
            })
        })
        .unwrap_or(0);
    let build_id = report.map(|r| r.build_id).unwrap_or(0);
    let pid = report.and_then(|r| r.pid);

    let request = montrs_dev_hotpatch::PatchRequest {
        capture_base: hotpatch_dir,
        exe: bin,
        workspace_target_dir,
        real_linker: Path::new(&linker),
        flavor,
        aslr_reference,
        build_id,
        pid,
    };
    match montrs_dev_hotpatch::build_patch(&request) {
        Ok(table) => {
            let entries = table.map.len();
            let lib = table.lib.display().to_string();
            if let Some(server) = server {
                server.hot_reload(table, build_id, pid);
            }
            println!(
                "Hot-patch: built patch {lib} with {entries} jump-table entries."
            );
        }
        Err(e) => eprintln!("Hot-patch: patch build failed: {e}"),
    }
}

/// Locate a wrapper binary by stem: `MONTRS_<STEM>_PATH` override, then `PATH`,
/// then next to the running executable.
fn resolve_wrapper(stem: &str) -> Option<PathBuf> {
    let override_key = format!(
        "MONTRS_{}_PATH",
        stem.to_uppercase().replace('-', "_")
    );
    if let Some(p) = std::env::var_os(&override_key) {
        return Some(PathBuf::from(p));
    }
    let bin = if cfg!(windows) {
        format!("montrs-{stem}.exe")
    } else {
        format!("montrs-{stem}")
    };
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let candidate = dir.join(&bin);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let candidate = dir.join(&bin);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

/// Ask rustc which linker it would use, so the link wrapper can forward to it.
fn discover_real_linker() -> Option<PathBuf> {
    // Run the probe in its own temp dir: `rustc --print=link-args` still
    // emits an executable next to the probe, which must not land in the project.
    let dir = std::env::temp_dir().join(format!(
        "montrs-linker-probe-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).ok()?;
    let probe = dir.join("probe.rs");
    std::fs::write(&probe, "fn main() {}").ok()?;

    let output = std::process::Command::new("rustc")
        .current_dir(&dir)
        .arg("--print=link-args")
        .arg("probe.rs")
        .output()
        .ok();
    let _ = std::fs::remove_dir_all(&dir);

    let output = output?;
    if !output.status.success() {
        return None;
    }
    montrs_dev_hotpatch::parse_linker_from_link_args(
        &String::from_utf8_lossy(&output.stdout),
    )
}

/// The host target triple cargo builds for (e.g. `x86_64-pc-windows-msvc`).
fn host_triple() -> Option<String> {
    let output = std::process::Command::new("rustc")
        .arg("-vV")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines()
        .find_map(|l| l.strip_prefix("host: ").map(|s| s.trim().to_string()))
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

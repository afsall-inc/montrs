// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `montrs serve` in dylib mode (dev-only).
//!
//! Builds the app library as a `cdylib` and runs a generic shell
//! (`montrs-dev-shell`) that loads it. On each change the CLI rebuilds, copies
//! the fresh dylib to a new filename, and points the shell's reload file at it;
//! the shell swaps the app in place, so the HTTP listener never restarts.

use super::{RebuildKind, is_server_only_rs};
use montrs_build::{BuildPipeline, Pipeline, reload::LiveReload};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::process::Command as TokioCommand;

pub async fn run(watch_workspace: bool) -> anyhow::Result<()> {
    let mut pipeline = Pipeline::from_root(Path::new("."))?;
    pipeline.release = false;
    // Build the WASM client with the matching `hot` profile so both halves emit
    // the same hot-reload markers (otherwise hydration would silently fail).
    pipeline.hot_reload = true;
    crate::command::resolve_pipeline_bins(&mut pipeline);

    // Include the browser hot-patch client in the WASM bundle, and capture rustc
    // invocations so we can find the changed crates' wasm objects for a patch.
    if !pipeline
        .meta
        .serve
        .lib_features
        .iter()
        .any(|f| f == "hotpatch")
    {
        pipeline
            .meta
            .serve
            .lib_features
            .push("hotpatch".to_string());
    }
    let capture_dir = pipeline.workspace_target_dir.join("montrs-hotpatch");
    if let Some(wrapper) = resolve_wrapper("rustc-wrapper") {
        unsafe {
            std::env::set_var("RUSTC_WORKSPACE_WRAPPER", &wrapper);
            std::env::set_var("MONTRS_HOTPATCH_DIR", &capture_dir);
        }
    }

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
        .unwrap_or("app")
        .to_string();
    let mut reload_port = pipeline.meta.serve.reload_port;

    let lib_name = pipeline
        .meta
        .serve
        .package
        .as_deref()
        .unwrap_or("app")
        .replace('-', "_");
    let cdylib = pipeline
        .workspace_target_dir
        .join("debug")
        .join(format!("{lib_name}{}", std::env::consts::DLL_SUFFIX));
    let shell_bin = resolve_shell_bin().ok_or_else(|| {
        anyhow::anyhow!(
            "montrs-dev-shell not found. Build it with `cargo build -p \
             montrs-dev-shell` or set MONTRS_DEV_SHELL_PATH."
        )
    })?;

    let run_dir = pipeline.workspace_target_dir.join("montrs-dylib");
    std::fs::create_dir_all(&run_dir)?;
    let reload_file = run_dir.join("current");

    // Watch only the app tree by default (see `dev_watch`).
    let watch_roots = super::dev_watch::watch_roots(&pipeline, watch_workspace);
    let watch_options = super::dev_watch::watch_options(&pipeline);

    // View roots scanned for `view!` macros so markup edits can be patched
    // live without an SSR recompile or shell reload.
    let view_roots = super::dev_watch::view_roots(&pipeline);

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

    // Browser hot-patch hub; the app's `/_dioxus` bridge proxies to it so the
    // WASM client can receive and apply jump tables without a reload.
    let hub = match montrs_dev_hotpatch::server::HotPatchServer::start(
        reload_port.saturating_add(1),
    )
    .await
    {
        Ok((server, port)) => {
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
    };

    let (tx, mut rx) =
        tokio::sync::mpsc::channel::<(RebuildKind, Vec<PathBuf>)>(1);
    let pipeline_arc = Arc::new(pipeline);
    let workspace_root = pipeline_arc
        .workspace_target_dir
        .parent()
        .map(|p| p.to_path_buf());
    let _watcher = tokio::task::spawn_blocking({
        let tx = tx.clone();
        let reload = reload.clone();
        move || {
            let mut patcher = montrs_hot_reload::ViewPatcher::new(
                &view_roots,
                workspace_root,
            );
            let _ = montrs_build::watch_paths_with(
                &watch_roots,
                watch_options,
                move |changed: &[PathBuf]| {
                    let mut rebuild = false;
                    let mut needs_frontend = false;
                    for path in changed {
                        match path.extension().and_then(|e| e.to_str()) {
                            Some("rs") => {
                                let view_only = patcher.is_view_only(path);
                                if let Some(patches) = patcher.patch(path)
                                    && let Ok(json) =
                                        serde_json::to_string(&patches)
                                    && let Some(r) = &reload
                                {
                                    r.view(json);
                                }
                                if !view_only {
                                    rebuild = true;
                                    if !is_server_only_rs(path) {
                                        needs_frontend = true;
                                    }
                                }
                            }
                            _ => {
                                rebuild = true;
                                needs_frontend = true;
                            }
                        }
                    }
                    if rebuild {
                        let kind = if needs_frontend {
                            RebuildKind::Full
                        } else {
                            RebuildKind::ServerOnly
                        };
                        let _ = tx.blocking_send((kind, changed.to_vec()));
                    }
                },
            );
        }
    });

    // Initial build + first copy.
    build(pipeline_arc.clone()).await?;
    let mut version = 0u32;
    let mut current = copy_versioned(&cdylib, &run_dir, &lib_name, version)?;
    std::fs::write(&reload_file, current.display().to_string())?;

    println!("Serving (dylib mode) on http://{addr}");
    let mut child = spawn_shell(
        &shell_bin,
        &addr,
        &site_root,
        &pkg_dir,
        &output_name,
        reload_port,
        &current,
        &reload_file,
    )?;
    println!("montrs-dev-shell started.");

    loop {
        tokio::select! {
            Some((kind, changed)) = rx.recv() => {
                println!("Change detected — rebuilding...");
                if let Some(r) = &reload { r.building(); }
                let result = match kind {
                    RebuildKind::Full => build(pipeline_arc.clone()).await,
                    RebuildKind::ServerOnly => build_server_only(pipeline_arc.clone()).await,
                };
                match result {
                    Ok(()) => {
                        version += 1;
                        current = copy_versioned(&cdylib, &run_dir, &lib_name, version)?;
                        std::fs::write(&reload_file, current.display().to_string())?;
                        println!("Rebuilt; asked the shell to reload {}.", current.display());
                        if let Some(hub) = &hub {
                            try_wasm_patch(&pipeline_arc, &changed, hub);
                        }
                        if let Some(r) = &reload {
                            r.build_ok();
                            r.notify();
                        }
                    }
                    Err(e) => {
                        eprintln!("Build error: {e}");
                        if let Some(r) = &reload { report_build_error(r, &e); }
                    }
                }
            }
            status = async { child.wait().await } => {
                eprintln!("montrs-dev-shell exited ({status:?}); restarting...");
                tokio::time::sleep(Duration::from_millis(300)).await;
                child = spawn_shell(
                    &shell_bin, &addr, &site_root, &pkg_dir, &output_name,
                    reload_port, &current, &reload_file,
                )?;
                println!("montrs-dev-shell restarted.");
            }
        }
    }
}

async fn build(pipeline: Arc<Pipeline>) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || pipeline.build_all())
        .await
        .map_err(|e| anyhow::anyhow!("build task panicked: {e}"))?
}

/// Rebuild only the SSR `cdylib` — the WASM client is already patched live by
/// the `view!` hot-reload watcher, so it stays valid. The app lib is already a
/// `cdylib`, so `build_server_only` produces the `.dll` the shell swaps in.
/// This is the fast path for server-only `.rs` edits in dylib mode.
async fn build_server_only(pipeline: Arc<Pipeline>) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || pipeline.build_server_only())
        .await
        .map_err(|e| anyhow::anyhow!("build task panicked: {e}"))?
}

fn copy_versioned(
    cdylib: &Path,
    run_dir: &Path,
    lib_name: &str,
    version: u32,
) -> anyhow::Result<PathBuf> {
    if !cdylib.is_file() {
        anyhow::bail!(
            "app cdylib not found at {}; does the app export montrs_app_entry?",
            cdylib.display()
        );
    }
    let dest = run_dir.join(format!(
        "{lib_name}-{version}{}",
        std::env::consts::DLL_SUFFIX
    ));
    if dest.exists() {
        let _ = std::fs::remove_file(&dest);
    }
    std::fs::copy(cdylib, &dest)?;
    Ok(dest)
}

#[allow(clippy::too_many_arguments)]
fn spawn_shell(
    shell_bin: &Path,
    addr: &str,
    site_root: &str,
    pkg_dir: &str,
    output_name: &str,
    reload_port: u16,
    dylib: &Path,
    reload_file: &Path,
) -> anyhow::Result<tokio::process::Child> {
    TokioCommand::new(shell_bin)
        .env("MONTRS_SITE_ADDR", addr)
        .env("MONTRS_SITE_ROOT", site_root)
        .env("MONTRS_SITE_PKG_DIR", pkg_dir)
        .env("MONTRS_OUTPUT_NAME", output_name)
        .env("MONTRS_RELOAD_PORT", reload_port.to_string())
        .env("MONTRS_APP_DYLIB", dylib)
        .env("MONTRS_RELOAD_FILE", reload_file)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to spawn montrs-dev-shell: {e}"))
}

fn resolve_shell_bin() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("MONTRS_DEV_SHELL_PATH") {
        return Some(PathBuf::from(p));
    }
    let name = if cfg!(windows) {
        "montrs-dev-shell.exe"
    } else {
        "montrs-dev-shell"
    };
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn report_build_error(reload: &LiveReload, err: &anyhow::Error) {
    reload.build_error(&err.to_string(), None, None, None, None);
}

/// Build and broadcast a WASM patch for the changed crates, if any.
fn try_wasm_patch(
    pipeline: &Pipeline,
    changed: &[PathBuf],
    hub: &montrs_dev_hotpatch::server::HotPatchServer,
) {
    let Some(wasm_ld) = montrs_dev_hotpatch::find_wasm_ld() else {
        return;
    };
    let output_name =
        pipeline.meta.serve.output_name.as_deref().unwrap_or("app");
    let fat = pipeline.pkg_dir.join(format!("{output_name}_bg.wasm"));
    if !fat.is_file() {
        return;
    }
    let wasm_deps = pipeline
        .workspace_target_dir
        .join("wasm32-unknown-unknown")
        .join("hot")
        .join("deps");
    let invocations = montrs_dev_hotpatch::read_rustc_invocations();
    let objects = match montrs_dev_hotpatch::changed_wasm_objects(
        &invocations,
        changed,
        &wasm_deps,
    ) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Hot-patch: wasm object selection failed: {e}");
            return;
        }
    };
    if objects.is_empty() {
        return;
    }

    let patch_out = pipeline.pkg_dir.join("montrs-patch.wasm");
    let pkg_component = pipeline
        .meta
        .serve
        .site_pkg_dir
        .trim_matches('/')
        .to_string();
    let url = format!("/{pkg_component}/montrs-patch.wasm");

    let request = montrs_dev_hotpatch::WasmPatchRequest {
        fat_wasm: &fat,
        objects: &objects,
        out: &patch_out,
        wasm_ld: &wasm_ld,
        url: &url,
    };
    match montrs_dev_hotpatch::build_wasm_patch(&request) {
        Ok(table) => {
            let entries = table.map.len();
            hub.hot_reload(table, 0, None);
            println!("Hot-patch: wasm patch with {entries} entries -> {url}");
        }
        Err(e) => eprintln!("Hot-patch: wasm patch failed: {e}"),
    }
}

/// Locate a wrapper binary by stem (PATH, then next to the running executable).
fn resolve_wrapper(stem: &str) -> Option<PathBuf> {
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

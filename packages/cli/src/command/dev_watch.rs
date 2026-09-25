// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared dev-loop watch configuration for `montrs serve` / `montrs watch`.
//!
//! By default the dev loop watches **only the app directory** (the one that
//! contains `montrs.toml`) plus its manifests — never the whole repository.
//! `montrs.toml [watch]` widens or narrows the set, and `--watch-workspace`
//! restores framework-contributor behaviour (watch the workspace `packages/`).

use montrs_build::{Pipeline, WatchOptions};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// Default source directories, relative to the app root, watched when present.
const DEFAULT_SOURCE_DIRS: &[&str] = &["app", "src", "style", "assets"];

/// Default manifests watched when present.
const DEFAULT_MANIFESTS: &[&str] = &["montrs.toml", "Cargo.toml"];

/// Resolve the set of paths the dev loop should watch.
///
/// `cli_watch_workspace` forces workspace-package watching (the
/// `--watch-workspace` flag); otherwise `[watch] workspace-packages` decides.
pub fn watch_roots(
    pipeline: &Pipeline,
    cli_watch_workspace: bool,
) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    let app_root = &pipeline.project_root;
    let watch = &pipeline.meta.watch;

    // Only watch the app tree. The project root itself is intentionally NOT
    // watched recursively: that is what dragged the whole repository (and its
    // `packages/` tree) into every rebuild.
    for candidate in DEFAULT_SOURCE_DIRS {
        let dir = app_root.join(candidate);
        if dir.exists() {
            roots.push(dir);
        }
    }
    for manifest in DEFAULT_MANIFESTS {
        let file = app_root.join(manifest);
        if file.exists() {
            roots.push(file);
        }
    }

    // Extra paths from `[watch] paths` (relative to the app root).
    for extra in &watch.paths {
        let path = app_root.join(extra);
        if path.exists() {
            roots.push(path);
        }
    }

    // Legacy `[serve] watch-additional-files`.
    for extra in &pipeline.meta.serve.watch_additional_files {
        let path = app_root.join(extra);
        if path.exists() {
            roots.push(path);
        }
    }

    // Framework contributor mode: watch the workspace `packages/` tree.
    let watch_packages = cli_watch_workspace || watch.workspace_packages;
    if watch_packages
        && let Some(ws_root) = pipeline.workspace_target_dir.parent()
    {
        let packages = ws_root.join("packages");
        if packages.exists() {
            roots.push(packages);
        }
    }

    // Follow local `path` dependencies so edits to sibling crates reload.
    if watch.follow_path_deps {
        for dep in collect_path_deps(app_root) {
            if dep.exists() {
                roots.push(dep);
            }
        }
    }

    // Never fall back to the whole project root: if nothing was found, watch
    // the app root only (scoped, non-recursive churn is still filtered).
    if roots.is_empty() {
        roots.push(app_root.clone());
    }

    roots.sort();
    roots.dedup();
    roots
}

/// Build [`WatchOptions`] from `montrs.toml [watch]`.
pub fn watch_options(pipeline: &Pipeline) -> WatchOptions {
    let watch = &pipeline.meta.watch;
    WatchOptions {
        exclude: watch.exclude.clone(),
        extensions: watch.extensions.clone(),
        debounce: watch
            .debounce_ms
            .map(std::time::Duration::from_millis)
            .unwrap_or_else(|| std::time::Duration::from_millis(200)),
    }
}

/// Roots scanned for `view!` macros so markup edits can be hot-patched.
pub fn view_roots(pipeline: &Pipeline) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    for candidate in ["app", "src"] {
        let dir = pipeline.project_root.join(candidate);
        if dir.exists() {
            roots.push(dir);
        }
    }
    if let Some(ws_root) = pipeline.workspace_target_dir.parent() {
        let ui = ws_root.join("packages").join("ui");
        if ui.exists() {
            roots.push(ui);
        }
    }
    roots
}

/// Collect local `path` dependencies declared in `Cargo.toml` files, starting
/// at `root` and following transitively (bounded, no `cargo` invocation).
fn collect_path_deps(root: &Path) -> Vec<PathBuf> {
    const MAX_DEPTH: usize = 8;
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut out: Vec<PathBuf> = Vec::new();
    let mut queue: Vec<(PathBuf, usize)> = vec![(root.to_path_buf(), 0)];

    while let Some((dir, depth)) = queue.pop() {
        if depth > MAX_DEPTH {
            continue;
        }
        let manifest = dir.join("Cargo.toml");
        let Ok(text) = std::fs::read_to_string(&manifest) else {
            continue;
        };
        for path in scan_path_deps(&text, &dir) {
            if visited.insert(path.clone()) {
                out.push(path.clone());
                queue.push((path, depth + 1));
            }
        }
    }
    out
}

/// Extract `path = "…"` dependency targets from a `Cargo.toml` body.
fn scan_path_deps(manifest_text: &str, manifest_dir: &Path) -> Vec<PathBuf> {
    let mut deps = Vec::new();
    for raw in manifest_text.lines() {
        let line = raw.trim();
        if line.starts_with('#') {
            continue;
        }
        let Some(idx) = line.find("path") else {
            continue;
        };
        let rest = &line[idx + "path".len()..];
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim();
        let Some(rest) = rest.strip_prefix('"') else {
            continue;
        };
        let Some(end) = rest.find('"') else {
            continue;
        };
        let rel = &rest[..end];
        if rel.is_empty() {
            continue;
        }
        deps.push(manifest_dir.join(rel));
    }
    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_path_deps_finds_path_dependencies() {
        let toml = r#"
[dependencies]
serde = "1"
montrs-core = { path = "../../packages/core" }
montrs-ui = { path = "../../packages/ui", features = ["csr"] }
# montrs-skip = { path = "nope" }
"#;
        let dir = Path::new("/app/app");
        let deps = scan_path_deps(toml, dir);
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&dir.join("../../packages/core")));
        assert!(deps.contains(&dir.join("../../packages/ui")));
    }
}

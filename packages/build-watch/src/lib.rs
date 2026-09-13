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

//! montrs-build-watch: File system watcher for MontRS projects.
//!
//! Watches a directory for changes and triggers a rebuild via the
//! `BuildPipeline` trait. Uses `notify` for cross-platform file watching
//! with built-in debouncing.

use anyhow::Result;
use montrs_build_core::BuildPipeline;
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

pub mod reload;

/// Path components that must never trigger a rebuild — build outputs, git
/// internals, and node_modules cause infinite rebuild loops otherwise.
fn is_ignored(path: &Path) -> bool {
    const IGNORED: &[&str] = &[
        "target",
        ".git",
        "node_modules",
        ".agent",
        ".opencode",
        ".references",
    ];
    path.components().any(|c| {
        IGNORED
            .iter()
            .any(|ig| c.as_os_str() == Path::new(ig).as_os_str())
    })
}

/// File extensions that can affect a build. Filtering to source-like files
/// keeps a workspace-wide watch from rebuilding on incidental churn (lock
/// files, editor temp files, logs) while still catching every real edit.
fn has_watched_extension(path: &Path) -> bool {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
    {
        Some(ext) => matches!(
            ext.as_str(),
            "rs" | "css"
                | "scss"
                | "sass"
                | "toml"
                | "html"
                | "htm"
                | "svg"
                | "json"
                | "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "webp"
                | "avif"
                | "ico"
                | "woff"
                | "woff2"
                | "ttf"
                | "otf"
                | "js"
                | "mjs"
                | "ts"
                | "txt"
                | "md"
        ),
        None => false,
    }
}

/// Whether a filesystem path should trigger a rebuild.
fn is_watched(path: &Path) -> bool {
    !is_ignored(path) && has_watched_extension(path)
}

/// Watch one or more directory trees for changes, invoking `on_change` with
/// the set of changed, build-relevant paths.
///
/// Uses debouncing: after the first change event, waits 200ms for more events
/// and coalesces their paths before calling the callback once.
pub fn watch_paths(
    paths: &[PathBuf],
    mut on_change: impl FnMut(&[PathBuf]) + Send + 'static,
) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res
                && matches!(
                    event.kind,
                    EventKind::Modify(_)
                        | EventKind::Create(_)
                        | EventKind::Remove(_)
                )
            {
                let changed: Vec<PathBuf> = event
                    .paths
                    .into_iter()
                    .filter(|p| is_watched(p))
                    .collect();
                if !changed.is_empty() {
                    let _ = tx.send(changed);
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )?;

    for path in paths {
        if path.exists() {
            watcher.watch(path, RecursiveMode::Recursive)?;
        }
    }

    let debounce = Duration::from_millis(200);
    loop {
        if let Ok(mut changed) = rx.recv() {
            while let Ok(more) = rx.recv_timeout(debounce) {
                changed.extend(more);
            }
            changed.sort();
            changed.dedup();
            on_change(&changed);
        }
    }
}

/// Watch a single directory tree. Convenience wrapper around [`watch_paths`].
pub fn watch_directory(
    path: &Path,
    on_change: impl FnMut(&[PathBuf]) + Send + 'static,
) -> Result<()> {
    watch_paths(&[path.to_path_buf()], on_change)
}

/// Watch a directory and rebuild the entire pipeline on changes.
///
/// Convenience wrapper around `watch_directory` that calls
/// `pipeline.build_all()` on each change.
pub fn watch_and_rebuild(
    path: &Path,
    pipeline: &'static impl BuildPipeline,
) -> Result<()> {
    watch_directory(path, move |_changed| {
        println!("Change detected — rebuilding...");
        if let Err(e) = pipeline.build_all() {
            eprintln!("Build error: {e}");
        } else {
            println!("Rebuild complete.");
        }
    })
}

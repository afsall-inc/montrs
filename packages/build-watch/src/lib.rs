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
use std::{path::Path, sync::mpsc, time::Duration};

/// Watch a directory for changes, triggering a rebuild via the pipeline.
///
/// Uses debouncing: after the first change event, waits 300ms for more
/// events before triggering the rebuild callback.
pub fn watch_directory(
    path: &Path,
    on_change: impl Fn() + Send + 'static,
) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res
                && matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_)
                )
            {
                let _ = tx.send(());
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    let debounce = Duration::from_millis(300);
    loop {
        if rx.recv().is_ok() {
            while rx.recv_timeout(debounce).is_ok() {}
            on_change();
        }
    }
}

/// Watch a directory and rebuild the entire pipeline on changes.
///
/// Convenience wrapper around `watch_directory` that calls
/// `pipeline.build_all()` on each change.
pub fn watch_and_rebuild(
    path: &Path,
    pipeline: &'static impl BuildPipeline,
) -> Result<()> {
    watch_directory(path, move || {
        println!("Change detected — rebuilding...");
        if let Err(e) = pipeline.build_all() {
            eprintln!("Build error: {e}");
        } else {
            println!("Rebuild complete.");
        }
    })
}

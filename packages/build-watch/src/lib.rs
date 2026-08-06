//! montrs-build-watch: File system watcher for MontRS projects.
//!
//! Watches a directory for changes and triggers a rebuild via the
//! `BuildPipeline` trait. Uses `notify` for cross-platform file watching
//! with built-in debouncing.

#[cfg(test)]
pub mod test_helpers;

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
            if let Ok(event) = res {
                if matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_)
                ) {
                    let _ = tx.send(());
                }
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
    pipeline: &'static (impl BuildPipeline + Send + Sync),
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
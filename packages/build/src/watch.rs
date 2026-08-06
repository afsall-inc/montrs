use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{path::Path, sync::mpsc, time::Duration};

/// Watch a directory for changes and trigger rebuilds.
pub fn watch_directory(
    path: &Path,
    on_change: impl Fn() + Send + 'static,
) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                // Debounce: only trigger on modify/create events
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

    // Debounce loop: trigger rebuild after a short delay
    let debounce = Duration::from_millis(300);
    loop {
        // Wait for first event
        if rx.recv().is_ok() {
            // Wait for more events (debounce)
            while rx.recv_timeout(debounce).is_ok() {}
            on_change();
        }
    }
}

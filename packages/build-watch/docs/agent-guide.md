# Agent Guide: montrs-build-watch

## Core Concepts
File system watcher with debounced rebuild triggers.

### Watching
- Uses the `notify` crate for cross-platform file system events.
- Watches the project source directory for changes.
- Filters out irrelevant events (e.g., `target/`, `.git/`).

### Debouncing
- Multiple rapid changes are coalesced into a single rebuild event.
- The debounce interval is configurable (default: 100ms).
- Ensures that save-all operations don't trigger redundant builds.

### Integration
- Composes with `build-serve` for live reload development.
- Fires a callback when a rebuild is needed.
- The callback is provided by the `BuildPipeline` trait.

## Important Rules
- File watching is cross-platform via the `notify` crate.
- Debouncing prevents unnecessary rebuilds during rapid changes.
- The watcher respects `.gitignore` patterns by default.
- Only file changes in the project directory are monitored.
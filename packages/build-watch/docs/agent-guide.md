# Build-Watch Package — Agent Guide

## Overview
`montrs-build-watch` provides file system watching for MontRS projects. It watches a directory for changes and triggers a rebuild via the `BuildPipeline` trait.

## Key Concepts
- **watch_directory**: Watches a path and triggers a callback on change events (debounced 300ms).
- **watch_and_rebuild**: Convenience wrapper that calls `pipeline.build_all()` on changes.

## Agent Usage
- Use `watch_directory(path, on_change)` for custom rebuild logic.
- Use `watch_and_rebuild(path, pipeline)` for the full pipeline rebuild.
- Events are debounced to avoid redundant rebuilds.

## Local Invariants
Read `docs/invariants.md` before modifying.
# Desktop Package — Agent Guide

## Overview
`montrs-desktop` provides the desktop platform shell for MontRS. It implements `PlatformAdapter` from `montrs-platform` and provides `run_webview` / `run_native` entry points.

## Key Concepts
- **DesktopAdapter**: Implements `PlatformAdapter` for desktop targets.
- **run_webview**: Opens a webview window (wry-based).
- **run_native**: Opens a native window (winit + wgpu-based).

## Agent Usage
- Use `DesktopAdapter::new()` to create the adapter.
- Use `run_webview(spec)` or `run_native(spec)` to launch the desktop app.

## Local Invariants
Read `docs/invariants.md` before modifying.
# Web Package — Agent Guide

## Overview
`montrs-web` implements `PlatformAdapter` from `montrs-platform` for browser/WASM targets. Uses `web-sys` and `wasm-bindgen` for DOM and browser API access.

## Key Concepts
- **WebAdapter**: Implements `PlatformAdapter` for web targets.
- **WASM-Only**: The adapter is only functional on `wasm32-unknown-unknown`.
- **No Leptos Dependency**: Uses raw `web-sys` bindings.

## Agent Usage
- Use `WebAdapter::new()` to create the adapter.
- `open_url` navigates the browser window.
- `set_title` sets `document.title`.

## Local Invariants
Read `docs/invariants.md` before modifying.
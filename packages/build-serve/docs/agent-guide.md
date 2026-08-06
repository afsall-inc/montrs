# Build-Serve Package — Agent Guide

## Overview
`montrs-build-serve` provides the dev server for MontRS. It serves static files from the build output directory using axum.

## Key Concepts
- **ServeConfig**: Contains `addr` and `site_root`.
- **serve_static**: Serves static files.
- **serve_with_callback**: Serves with an `on_ready` callback.

## Agent Usage
- Use `serve_static(ServeConfig)` to start the dev server.
- Use `serve_with_callback` when you need to know when the server is ready.

## Local Invariants
Read `docs/invariants.md` before modifying.
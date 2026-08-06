# Platform Package — Agent Guide

## Overview
`montrs-platform` is the layer-0 platform abstraction crate. It defines the `Target` enum and `PlatformAdapter` trait that all platform-specific packages implement.

## Key Concepts
- **Target enum**: `Web`, `Desktop`, `Mobile`, `Tui` — the execution environment.
- **PlatformAdapter trait**: `target()`, `open_url()`, `set_title()`, `set_size()`, `description()`.
- **NoopPlatformAdapter**: Default no-op implementation for testing.

## Agent Usage
- Implement `PlatformAdapter` for new platform targets.
- Use `Target` for conditional compilation logic.
- Use `NoopPlatformAdapter` in tests.

## Local Invariants
Read `docs/invariants.md` before modifying.
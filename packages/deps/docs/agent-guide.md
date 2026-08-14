# montrs-deps — Agent Guide

## Overview
Manages project dependencies from the `[deps]` section of `montrs.toml`. Checks if lockfiles and build outputs are fresh relative to source files.

## Key Concepts
- **DepSpec**: `provider:target` with optional `auto` and `options`
- **DepsManager**: Loads, lists, and checks dependencies
- **Freshness**: `Fresh`, `OutputsMissing`, `Stale(reason)`, `Forced`
- **Source hash**: SHA256 over all source files for cache invalidation

## Agent Usage
- `DepSpec::parse("cargo:ripgrep@14.0.0", None)` to parse a dep key
- `DepsManager::load_from_config(&raw)` to load deps from montrs.toml
- `manager.check_freshness("cargo:ripgrep")` to check if up-to-date
- `known_lockfiles()` for common lockfile patterns

## Local Invariants
Read `docs/invariants.md` before modifying.
# Build-Core Package — Agent Guide

## Overview
`montrs-build-core` defines the `BuildPipeline` trait and `BuildConfig` types that orchestrate the MontRS build process. The concrete `Pipeline` struct lives in `montrs-build`.

## Key Concepts
- **BuildPipeline trait**: `build_server`, `build_frontend`, `process_tailwind`, `copy_assets`, `generate_index_html`, `build_all`.
- **BuildStep enum**: `Server`, `Frontend`, `Tailwind`, `Assets`, `IndexHtml`.
- **ProjectConfig**: Loads metadata from `montrs.toml`.

## Agent Usage
- Implement `BuildPipeline` to add a custom build process.
- Use `BuildStep` to describe the stages of a build.
- Call `ProjectConfig::from_root()` to load project config.

## Local Invariants
Read `docs/invariants.md` before modifying.
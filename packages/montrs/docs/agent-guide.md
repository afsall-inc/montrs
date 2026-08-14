# MontRS Facade Package — Agent Guide

## Overview
`montrs` is the facade crate for the entire MontRS framework. It re-exports all public APIs from sub-packages and provides convenience re-exports.

## Key Concepts
- **prelude module**: Convenience re-exports of commonly used types.
- **Feature Flags**: Each sub-package is feature-gated (e.g., `cli`, `ui`, `orm`, `icons`).
- **Binary Target**: The `montrs` CLI binary is built when the `cli` feature is enabled.

## Agent Usage
- Use `montrs::prelude::*` for quick access to common types.
- Enable features selectively: `montrs = { features = ["cli", "ui"] }`.

## Local Invariants
Read `docs/invariants.md` before modifying.
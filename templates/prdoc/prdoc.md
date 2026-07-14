---
title: "Short description of the change"
pr: 0
author: "@username"
status: draft
packages: ["core"]
breaking: false
needs-review: []
audience: ["app_dev"]
crates:
  - name: core
    bump: patch
---

## Summary

A human-readable summary of what this PR does and why.

## Changes
### Packages Affected
- **core**: Description of changes to this package.

## Agent Instructions
### Verification
1. Run `cargo test --workspace` — all tests must pass.
2. Run `cargo clippy --workspace -- -D warnings` — no warnings.
3. Run `montrs agent check` — no invariant violations.

### Review Focus


## Migration Notes


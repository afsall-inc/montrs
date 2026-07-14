---
title: "Asymptote"
pr: 78
author: "balqaasem"
status: draft
packages: ["agent", "cli"]
breaking: true
needs-review: ["architecture"]
audience: ["framework_dev", "agent_user", "operator"]
crates:
  - name: agent
    bump: minor
    validate: true
  - name: cli
    bump: major
    validate: true
---

## Summary

- Contains breaking changes
- Adds new public API surface in 7 file(s)
- Modifies 8 source file(s) (+1690/-24)

## Changes
### Packages Affected
- **agent** (bump: minor): Changes to this package.
- **cli** (bump: major): Changes to this package.

## Agent Instructions
### Verification
1. Run `cargo test --workspace` — all tests must pass.
2. Run `cargo clippy --workspace -- -D warnings` — no warnings.
3. Run `montrs agent check` — no invariant violations.

### Review Focus

## Migration Notes

This PR introduces breaking changes. Review the public API modifications carefully.

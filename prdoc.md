---
title: "refactor(prdoc): improve readability with structured sections, dedup, proper YAML"
pr: 80
author: balqaasem
status: draft
packages:
  - agent
  - cli
  - prdoc
breaking: true
needs-review:
  - architecture
  - migration
audience:
  - framework_dev
  - agent_user
  - operator
crates:
  - name: agent
    bump: minor
    validate: true
  - name: cli
    bump: minor
    validate: true
  - name: prdoc
    bump: major
    validate: true
---

## Summary

- summary.rs: restructured generate_rich_summary into sections (Key Changes, Package Breakdown). Added dedup logic (filter_and_dedup_api_changes) to remove items that appear in both additions and removals. Added noise filtering for common trait impl methods (as_str, new, etc.) and test artifacts. Grouped API changes by package and type with per-category caps at 12 items (split_and_ellipsis). - generator.rs: replaced Rust Debug formatting with proper YAML (indented lists, yaml_kv helper with esca...

### Key Changes
**Removed**
- **project**: `AuthPlate`, `CounterState`, `CreateUserInput`, `MyBench`, `ProjectConfig`, `Route`, `User`, `UserLoader`, `UserRoute`
### Package Breakdown
- **agent** (minor): 0 source file(s) (+4/-0)
- **cli** (minor): 2 source file(s) (+10/-1)
- **prdoc** (major): 6 source file(s) (+537/-267)


**Breaking:** This change modifies the public API in a backward-incompatible way.

## Changes
### Packages Affected
- **agent** (minor): +4/-0
- **cli** (minor): (2 source file(s)) +10/-1
- **prdoc** (major): (6 source file(s)) +537/-267

## Agent Instructions
### Verification
1. Run `cargo test --workspace` — all tests must pass.
2. Run `cargo clippy --workspace -- -D warnings` — no warnings.
3. Run `montrs agent check` — no invariant violations.

### Review Focus
- Architecture: verify structural integrity of public API changes.
- Migration: validate that breaking changes are documented.

## Migration Notes

This PR introduces breaking changes to: prdoc.
From PR description:
Migration Notes with specific breaking package names. Better title derivation from PR context when available. Per-package descriptions in Packages Affected section.
Review the public API modifications carefully before merging.

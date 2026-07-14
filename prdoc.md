---
title: "Add skills, prdoc, and agent CLI subcommands"
author: "@balqaasem"
status: draft
packages: [agent, cli, montrs]
breaking: false
needs-review: [architecture, agent]
---

## Summary

Extends MontRS with agentic capabilities: a composable skills system, structured PR documentation (prdoc.md), new agent CLI subcommands (skills, snapshot, resolve, prdoc, rules export/list), and updated documentation.

## Changes
### Packages Affected
- **agent**: Added `SkillManifest`, `discover_skills()`, `skills_to_tools_json()`, `@agent-skill` marker, `PrDoc` struct, `parse_prdoc()`/`load_prdoc()`/`validate_prdoc()`, rules export to `.trae/` and `.cursorrules`.
- **cli**: Added `AgentSubcommand` variants (`Skills`, `Snapshot`, `Resolve`, `Prdoc`), `RulesSubcommand::Export/List`, `--json` flag on check/doctor/list-errors, removed `cargo-montrs` binary.
- **montrs**: Switched from `cargo-montrs` to `montrs` as the sole binary.

## Agent Instructions
### Verification
1. Run `cargo +nightly-2026-02-18 fmt --check` — no formatting issues.
2. Run `cargo +nightly-2026-02-18 clippy --workspace -- -D warnings` — no warnings.
3. Run `cargo +nightly-2026-02-18 test --workspace` — all tests pass.
4. Run `cargo run --package montrs-cli -- agent prdoc --validate` — prdoc.md is valid.

### Review Focus
- Skill manifests follow the schema in `docs/agent/skills.md`.
- CLI subcommands follow existing patterns in `packages/cli/src/lib.rs`.
- PrDoc YAML frontmatter parsing uses `serde_yaml`, not `toml`.

## Migration Notes
- `cargo-montrs` binary removed; use `montrs` instead.
- `.agent/rules/` is now the canonical rules location; `.trae/` and `.cursorrules` are export targets.

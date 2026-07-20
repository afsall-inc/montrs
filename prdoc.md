---
title: Asymptote
pr: 82
author: balqaasem
status: draft
packages:
  - agent
  - agentignore
  - cli
  - haptics
  - montrs
  - prdoc
breaking: true
needs-review:
  - architecture
  - migration
  - design
audience:
  - framework_dev
  - agent_user
crates:
  - name: agent
    bump: major
    validate: true
  - name: agentignore
    bump: minor
    validate: true
  - name: cli
    bump: major
    validate: true
  - name: haptics
    bump: minor
    validate: true
  - name: montrs
    bump: minor
    validate: true
  - name: prdoc
    bump: major
    validate: true
---

## Summary

Adds agentignore): add .agentignore support with IDE-specific export …, agents): add comprehensive agent guide documentation, haptics): cross-platform haptic feedback with platform providers…. Fixes prdoc): distinguish moved items from removed in prdoc generation, prdoc): add missing lifetime annotation to moved_items parameter. Chore: auto-generate prdoc.md.

### Key Changes
**Added**
- **cli**: `MontrsCli` structs; `AgentSubcommand`, `CargoCli`, `ChangelogSubcommand`, `Commands`, `GenerateSubcommand`, `IgnoreSubcommand`, `McpSubcommand`, `PrdocSubcommand`, `RulesSubcommand` enums; `haptics`, `impact_heavy`, `impact_light`, `impact_medium`, `is_supported`, `main_entry`, `montrs_cli`, `plate`, `route`, `run`, `selection` functions
- **project**: `Route` traits; `AuthPlate`, `CounterState`, `CreateTodoInput`, `CreateUserInput`, `ProjectConfig`, `User`, `UserLoader` structs; `foo`, `old_function` functions
- **agent**: `export_rules_for_opencode`, `get_framework_invariants` functions
- **agentignore**: `AgentIgnore` structs; `check_path`, `create_from_gitignore`, `export_for_ide`, `is_ignored`, `load`, `patterns` functions
- **haptics**: `HapticsProvider` traits; `DesktopHapticsProvider`, `HapticsConfig`, `MobileHapticsProvider`, `WebHapticsProvider` structs; `HapticsTarget`, `ImpactStyle` enums; `create_haptics_provider` functions
- **prdoc**: `DiffAnalysis`, `FileChange`, `MovedItem`, `PrContext`, `PublicApiChange`, `SummaryContext` structs; `ChangeCategory`, `FileCategory` enums; `analyze_commits`, `analyze_diff`, `classify_commit`, `extract_public_api_from_diff`, `gather_pr_context_from_gh`, `generate_prdoc`, `generate_rich_summary`, `get_commit_messages_for_range`, `get_diff_for_pr`, `get_diff_for_range`, `render_prdoc`, `render_prdoc_rich` functions
### Package Breakdown
- **agent** (major): 2 source file(s) (+110/-76)
- **agentignore** (minor): 1 source file(s) (+161/-0)
- **cli** (major): 3 source file(s) (+955/-832)
- **haptics** (minor): 6 source file(s) (+484/-0)
- **montrs** (minor): 1 source file(s) (+74/-63)
- **prdoc** (major): 3 source file(s) (+1774/-1515)


**Breaking:** This change modifies the public API in a backward-incompatible way.

## Changes
### Packages Affected
- **agent** (major): (2 source file(s)) +110/-76
- **agentignore** (minor): (1 source file(s)) +161/-0
- **cli** (major): (3 source file(s)) +955/-832
- **haptics** (minor): (6 source file(s), 1 test file(s)) +484/-0
- **montrs** (minor): (1 source file(s)) +74/-63
- **prdoc** (major): (3 source file(s)) +1774/-1515

## Agent Instructions
### Verification
1. Run `cargo test --workspace` — all tests must pass.
2. Run `cargo clippy --workspace -- -D warnings` — no warnings.
3. Run `montrs agent check` — no invariant violations.

### Review Focus
- Architecture: verify structural integrity of public API changes.
- Migration: validate that breaking changes are documented.
- Design: confirm new types and traits follow project conventions.

## Migration Notes

This PR introduces breaking changes to: agent, cli, prdoc.
Review the public API modifications carefully before merging.

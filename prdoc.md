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
---

## Summary

Adds agentignore): add .agentignore support with IDE-specific export …, agents): add comprehensive agent guide documentation, haptics): cross-platform haptic feedback with platform providers….

### Key Changes
**Added**
- **agent**: `export_rules_for_opencode` functions
- **haptics**: `HapticsProvider` traits; `DesktopHapticsProvider`, `HapticsConfig`, `MobileHapticsProvider`, `WebHapticsProvider` structs; `HapticsTarget`, `ImpactStyle` enums; `create_haptics_provider` functions
- **agentignore**: `AgentIgnore` structs; `check_path`, `create_from_gitignore`, `export_for_ide`, `is_ignored`, `load`, `patterns` functions
- **cli**: `IgnoreSubcommand` enums; `haptics`, `impact_heavy`, `impact_light`, `impact_medium`, `is_supported`, `selection` functions
**Removed**
- **project**: `AuthPlate`, `CounterState`, `CreateTodoInput`, `CreateUserInput`, `ProjectConfig`, `Route`, `User`, `UserLoader`
- **agent**: `get_framework_invariants`
- **cli**: `AgentSubcommand`, `CargoCli`, `ChangelogSubcommand`, `Commands`, `GenerateSubcommand`, `McpSubcommand`, `MontrsCli`, `PrdocSubcommand`, `RulesSubcommand`, `main_entry`, `montrs_cli`, `plate`; ...and 2 more
### Package Breakdown
- **agent** (major): 2 source file(s) (+110/-76)
- **agentignore** (minor): 1 source file(s) (+161/-0)
- **cli** (major): 3 source file(s) (+955/-832)
- **haptics** (minor): 6 source file(s) (+484/-0)
- **montrs** (minor): 1 source file(s) (+74/-63)


**Breaking:** This change modifies the public API in a backward-incompatible way.

## Changes
### Packages Affected
- **agent** (major): (2 source file(s)) +110/-76
- **agentignore** (minor): (1 source file(s)) +161/-0
- **cli** (major): (3 source file(s)) +955/-832
- **haptics** (minor): (6 source file(s), 1 test file(s)) +484/-0
- **montrs** (minor): (1 source file(s)) +74/-63

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

This PR introduces breaking changes to: agent, cli.
Review the public API modifications carefully before merging.

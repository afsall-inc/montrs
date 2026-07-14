# PRDoc: Pull Request Documentation

PRDoc is a structured markdown document that accompanies every pull request. It provides machine-readable context about what changed, why, and what needs verification — enabling agents to understand and review PRs autonomously.

## Format

Create a `prdoc.md` in the root of your PR branch:

```markdown
---
title: "Short description of the change"
pr: 1234
author: "@username"
status: draft
packages: [core, cli]
breaking: false
needs-review: [architecture, agent]
---

## Summary
A human-readable summary of what this PR does and why.

## Changes
### Packages Affected
- **core**: Added new `SkillManifest` struct for skill definitions.
- **cli**: Added `montrs agent skills` subcommand.

## Agent Instructions
### Verification
1. Run `cargo test --workspace` — all tests must pass.
2. Run `cargo clippy --workspace -- -D warnings` — no warnings.
3. Run `montrs agent check` — no invariant violations.

### Review Focus
- Check that new skill manifests follow the schema in `docs/agent/skills.md`.
- Verify that CLI subcommands follow existing patterns.

## Migration Notes
None. This is a purely additive change.
```

## Schema

| Field | Required | Description |
|-------|----------|-------------|
| `title` | yes | Short description |
| `pr` | no | PR number (added after creation) |
| `author` | yes | GitHub username or handle |
| `status` | yes | `draft`, `review`, `approved`, `merged` |
| `packages` | yes | List of affected package names |
| `breaking` | yes | Whether this introduces breaking changes |
| `needs-review` | no | Areas that need human review |

## Agent Integration

The `montrs agent prdoc` command validates and displays prdoc information. PRDoc files are included in the agent snapshot under `documentation_snippets["prdoc"]`.

## CI Integration

Add to your CI pipeline:
```yaml
- name: Validate PRDoc
  if: github.event_name == 'pull_request'
  run: montrs agent prdoc --validate
```
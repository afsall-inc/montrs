# PRDoc: Pull Request Documentation

PRDoc is a structured markdown document that accompanies every pull request. It provides machine-readable context about what changed, why, and what needs verification — enabling agents to understand and review PRs autonomously. PRDocs also drive automated changelog generation and per-crate version bumping.

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
audience: [framework_dev, app_dev]
crates:
  - name: core
    bump: minor
  - name: cli
    bump: patch
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
| `audience` | no | Who is affected: `app_dev`, `framework_dev`, `agent_user`, `operator` |
| `crates` | yes | Per-crate bump levels (see below) |

## Per-Crate SemVer Bumps

Each affected crate must have an entry in the `crates` section with a bump level:

| Bump | When to use |
|------|-------------|
| `major` | Breaking public API changes (removals, signature changes) |
| `minor` | New public API additions (new functions, structs, traits) |
| `patch` | Bug fixes or internal changes with no API change |
| `none` | No observable change (docs, CI, comments) |

The `validate` field per crate (default: `true`) controls whether CI semver checks enforce the bump. Set `validate: false` to override an incorrect CI recommendation, with justification in the PR description.

### Example

```yaml
crates:
  - name: montrs-core
    bump: major
  - name: montrs-cli
    bump: minor
    validate: false
```

## Audience

| Audience | Who they are |
|----------|-------------|
| `app_dev` | People building applications with MontRS |
| `framework_dev` | People working on the MontRS framework itself |
| `agent_user` | Agents consuming the machine-readable APIs |
| `operator` | People running CI, deployments, or infrastructure |

## Auto-Generation

PRDocs can be auto-generated from PR context using the CLI:

```bash
# From a PR number (uses gh CLI)
montrs agent prdoc generate --pr 42

# From a local diff file
montrs agent prdoc generate --from-diff changes.diff

# From a git commit range
montrs agent prdoc generate --from-commits main..HEAD

# With embedding-based classification
montrs agent prdoc generate --pr 42 --embed

# Overwrite existing prdoc.md
montrs agent prdoc generate --pr 42 --force
```

The generator uses a hybrid approach:
1. **Rule-based analysis**: Parses the diff to detect packages, public API changes, and breaking changes
2. **Embedding classification**: Keyword-based cosine similarity against change category prototypes (NewFeature, BugFix, BreakingChange, Refactor, Documentation, Internal)
3. **Template rendering**: Fills structured prdoc.md from analysis results

## CLI Commands

```bash
# Display prdoc as JSON
montrs agent prdoc show

# Validate prdoc schema
montrs agent prdoc validate

# Auto-generate prdoc
montrs agent prdoc generate --pr <number>
```

## CI Integration

PRDocs are automatically generated and validated in CI:

```yaml
# .github/workflows/prdoc.yml runs on PR open/sync
- name: Generate PRDoc
  run: montrs agent prdoc generate --pr ${{ github.event.pull_request.number }} --force

- name: Validate PRDoc
  run: montrs agent prdoc validate
```

## Changelog Generation

When a release is cut, prdocs from merged PRs are aggregated into a CHANGELOG.md:

```bash
# Generate changelog from prdocs since last release
montrs agent changelog generate --from v0.1.0

# Compute version bumps
montrs agent changelog bump --current 0.1.0

# Dry-run version bumps
montrs agent changelog bump --dry-run

# Verify all PRs have prdocs
montrs agent changelog verify
```

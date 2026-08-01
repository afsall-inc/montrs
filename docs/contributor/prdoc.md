# PRDoc: Pull Request Documentation

PRDoc is a structured YAML document that accompanies every pull request. It provides machine-readable context about what changed, why, and what needs verification — enabling agents to understand and review PRs autonomously. PRDocs also drive automated changelog generation and per-crate version bumping.

PRDocs are managed by [`changelogger-cli`](https://crates.io/crates/changelogger-cli) — install via `cargo install changelogger-cli`.

## Format

Create a `prdoc/pr_<number>.prdoc` file in the repo root:

```yaml
---
title: "Short description of the change"

doc:
  - audience: Developer
    description: |
      A human-readable summary of what this PR does and why.

crates:
  - name: montrs-core
    bump: minor
  - name: montrs-cli
    bump: patch
---
```

## Audience

Changelogger uses three audience values. These map to MontRS's previous convention as follows:

| Changelogger value | MontRS legacy value | Who they are |
|---|---|---|
| `Developer` | `Framework Dev` | People working on the MontRS framework itself |
| `User` | `App Dev` | People building applications with MontRS |
| `Operator` | `Operator` | People running CI, deployments, or infrastructure |

> Note: The legacy `Agent User` audience has been merged into `Developer` — agents are treated as developer tooling.

## Per-Crate SemVer Bumps

Each affected crate must have an entry in the `crates` section with a bump level:

| Bump | When to use |
|------|-------------|
| `major` | Breaking public API changes (removals, signature changes) |
| `minor` | New public API additions (new functions, structs, traits) |
| `patch` | Bug fixes or internal changes with no API change |
| `none` | No observable change (docs, CI, comments) |

### Example

```yaml
crates:
  - name: montrs-core
    bump: major
  - name: montrs-cli
    bump: minor
```

## CLI Commands

```bash
# Scaffold config and directory
changelogger prdoc init

# Generate prdoc from PR context (uses gh CLI)
changelogger prdoc generate --pr <number>

# Validate all prdoc files
changelogger prdoc validate

# Validate with backport branch check
changelogger prdoc validate --branch stable

# Display a prdoc as JSON
changelogger prdoc show prdoc/pr_42.prdoc

# Generate CHANGELOG.md from prdocs
changelogger changelog generate --from v0.1.0

# Compute next version bumps
changelogger changelog bump --current 0.1.0

# Verify all commits have prdocs
changelogger changelog verify --from v0.1.0
```

## CI Integration

PRDocs are automatically generated and validated in CI:

```yaml
# .github/workflows/prdoc.yml runs on PR open/sync
- name: Generate prdoc from PR context
  env:
    GH_TOKEN: ${{ github.token }}
  run: changelogger prdoc generate --pr ${{ github.event.pull_request.number }} --force

- name: Validate prdoc
  env:
    GH_TOKEN: ${{ github.token }}
  run: changelogger prdoc validate
```
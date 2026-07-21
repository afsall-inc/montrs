# PRDoc: Pull Request Documentation

PRDoc is a structured YAML document that accompanies every pull request. It provides machine-readable context about what changed, why, and what needs verification. PRDocs drive automated changelog generation and per-crate version bumping.

## Philosophy

**PRDocs are skeletons requiring human editing.** The generator detects metadata (packages, audience, bump level) from the git diff, but leaves descriptions as `...` placeholders for the author to fill in. This ensures meaningful, human-written documentation rather than LLM-generated rubbish.

## Format

Create a `prdoc/pr_<number>.prdoc` in the root of your PR branch:

```yaml
---
title: Short description of the change

doc:
  - audience: Framework Dev
    description: |
      Describe the change from the framework developer perspective.

crates:
  - name: montrs-core
    bump: minor
  - name: montrs-cli
    bump: patch
---
```

## Schema

| Field | Required | Description |
|-------|----------|-------------|
| `title` | yes | Short description of the change |
| `author` | no | GitHub username (auto-detected from PR) |
| `pr` | no | PR number (auto-detected from PR) |
| `doc` | yes | Array of audience-specific documentation sections |
| `doc[].audience` | yes | Target audience (see below) |
| `doc[].description` | yes | Description of the change for this audience |
| `doc[].title` | no | Optional title override for this audience |
| `crates` | yes | Per-crate bump levels (see below) |
| `crates[].name` | yes | Crate name |
| `crates[].bump` | yes | SemVer bump level |
| `crates[].validate` | no | Whether CI validates this bump (default: `true`) |
| `crates[].note` | no | Optional note about the crate change |
| `migrations` | no | Database and runtime migrations |
| `migrations.db` | no | Array of database migration entries |
| `migrations.runtime` | no | Array of runtime migration entries |
| `host_functions` | no | Array of host functions involved |

## Audiences

Pick one or more audiences:

| Audience | Who They Are |
|----------|-------------|
| `Framework Dev` | Contributors to the MontRS framework itself |
| `App Dev` | Developers building applications with MontRS |
| `Agent User` | Users of agent/automation features |
| `Operator` | Operators running MontRS infrastructure |

## Per-Crate SemVer Bumps

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
    note: "Removed deprecated `old_function`"
  - name: montrs-cli
    bump: minor
    validate: false
```

## Migrations and Host Functions

When your PR introduces database migrations, runtime migrations, or host function changes, document them:

```yaml
migrations:
  db:
    - name: add_user_table
      description: Adds user table with email column
  runtime:
    - description: Migrates old config format to new format
      reference: "migration_v2.rs"

host_functions:
  - name: ext_new_host_function
    description: New host function for X
    notes: Only available on wasm32 targets
```

## Auto-Generation

PRDoc skeletons can be auto-generated from PR context:

```bash
# From a PR number (uses gh CLI)
montrs agent prdoc generate --pr 42 --bump minor --audience app_dev

# Overwrite existing prdoc
montrs agent prdoc generate --pr 42 --bump minor --audience framework_dev --force
```

The generator:
1. Fetches PR info from GitHub API (title, body, author)
2. Downloads the PR diff
3. Parses the workspace with `cargo metadata` to find modified crates
4. Creates a skeleton in `prdoc/pr_<number>.prdoc` with `...` placeholders
5. **You must edit the descriptions** — the generator only fills metadata

## Validation

```bash
# Validate a prdoc file
montrs agent prdoc validate prdoc/pr_42.prdoc

# Validate with backport branch rules (major bumps require validate: false)
montrs agent prdoc validate --branch stable/v1 prdoc/pr_42.prdoc
```

Validation checks:
- Title is not `...`
- At least one doc section with non-`...` description
- At least one crate listed
- Crate names exist in the workspace
- Schema compliance
- On stable/release branches: major bumps require `validate: false`

## CLI Commands

```bash
# Display prdoc as JSON
montrs agent prdoc show prdoc/pr_42.prdoc

# Validate prdoc schema
montrs agent prdoc validate prdoc/pr_42.prdoc

# Auto-generate prdoc skeleton
montrs agent prdoc generate --pr 42 --bump minor --audience app_dev

# Validate with backport rules
montrs agent prdoc validate --branch stable/v1 prdoc/pr_42.prdoc
```

## CI Integration

PRDocs are validated in CI:

```yaml
# .github/workflows/prdoc.yml runs on PR open/sync
- name: Validate PRDoc
  run: montrs agent prdoc validate prdoc/pr_${{ github.event.pull_request.number }}.prdoc

- name: Validate backport rules
  if: startsWith(github.base_ref, 'stable') || startsWith(github.base_ref, 'release')
  run: montrs agent prdoc validate --branch ${{ github.base_ref }} prdoc/pr_${{ github.event.pull_request.number }}.prdoc
```

## Changelog Generation

When a release is cut, prdocs from merged PRs are aggregated into audience-specific changelogs:

```bash
# Generate per-audience changelog from prdocs
montrs agent changelog generate --from v0.1.0

# This produces separate sections for:
# - Framework Dev
# - App Dev
# - Agent User
# - Operator
```

## Template

```yaml
---
title: ...

doc:
  - audience: ...
    description: |
      ...

crates: [ ]
---
```

## Schema File

See `prdoc/schema_user.json` for the full JSON Schema definition.
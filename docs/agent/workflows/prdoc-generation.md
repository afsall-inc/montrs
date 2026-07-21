# PRDoc Generation Workflow

This workflow ensures accurate PR documentation following the polkadot-sdk style.

## Philosophy

**PRDocs are skeletons requiring human editing.** The generator detects metadata (packages, audience, bump level) from the git diff, but leaves descriptions as `...` placeholders for the author to fill in. This ensures meaningful, human-written documentation rather than LLM-generated rubbish.

## Steps

### 1. Generate Skeleton PRDoc

```bash
montrs agent prdoc generate --pr <number> --bump <level> --audience <audience>
```

Required parameters:
- `--pr` : PR number
- `--bump` : `major`, `minor`, `patch`, or `none`
- `--audience` : `app_dev`, `framework_dev`, `agent_user`, or `operator`
- `--force` : Overwrite existing prdoc file

Example:
```bash
montrs agent prdoc generate --pr 82 --bump minor --audience app_dev
```

This creates `prdoc/pr_82.prdoc` with:
- Modified crates detected from the git diff
- PR title and body from GitHub API
- `...` placeholders for descriptions

### 2. Fill In Descriptions

Edit the generated `prdoc/pr_<number>.prdoc` and replace all `...` with meaningful content:

```yaml
---
title: Add deferred dispatch support

doc:
  - audience: Framework Dev
    description: |
      Extends pallet-whitelist with automatic deferred dispatch...
      [Write a detailed technical description for this audience]

crates:
  - name: montrs-core
    bump: minor
---
```

### 3. Choose Audiences

Pick one or more audiences:

| Audience | Who They Are |
|----------|-------------|
| Framework Dev | Contributors to MontRS framework |
| App Dev | Developers building apps with MontRS |
| Agent User | Users of agent/automation features |
| Operator | Operators running MontRS infrastructure |

### 4. Set Bump Levels

| Bump | When to Use |
|------|-------------|
| `major` | Breaking public API changes (removals, signature changes) |
| `minor` | New public API additions (new functions, structs, traits) |
| `patch` | Bug fixes or internal changes with no API change |
| `none` | No observable change (docs, CI, comments) |

Use `validate: false` to override CI SemVer checks with justification.

### 5. Add Migrations (If Applicable)

```yaml
migrations:
  db:
    - name: add_user_table
      description: Adds user table with email column
  runtime:
    - description: Migrates old config format to new format
      reference: "migration_v2.rs"
```

### 6. Validate PRDoc

```bash
montrs agent prdoc validate prdoc/pr_<number>.prdoc
```

Checks:
- Title is not `...`
- At least one doc section with non-`...` description
- At least one crate listed
- Crate names exist in workspace
- Schema compliance

### 7. Validate Backport Rules

On stable/release branches, major bumps require `validate: false`:

```bash
montrs agent prdoc validate --branch stable/v1 prdoc/pr_<number>.prdoc
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

## Schema

See `prdoc/schema_user.json` for the full JSON Schema definition.
# PRDoc Generation Workflow

This workflow ensures accurate PR documentation following the polkadot-sdk style.

## Philosophy

**PRDocs are skeletons requiring human editing.** The generator detects metadata (packages, audience, bump level) but leaves descriptions as `...` placeholders for the author to fill in. This ensures meaningful, human-written documentation rather than LLM-generated rubbish.

## Steps

### 1. Generate Skeleton PRDoc

```bash
montrs agent prdoc generate --pr <number>
```

Or from a local diff:

```bash
montrs agent prdoc generate --from-diff changes.diff
```

This creates a skeleton with:
- Detected packages/crates
- Inferred audience
- Suggested bump level
- `...` placeholders for descriptions

### 2. Fill In Descriptions

Edit the generated `prdoc.md` and replace all `...` with meaningful content:

```yaml
title: Add deferred dispatch support

doc:
  - audience: Framework Dev
    description: |
      Extends `pallet-whitelist` with automatic deferred dispatch...
      [Write a detailed technical description for this audience]
      
  - audience: App Dev
    description: |
      [Different description for app developers, if applicable]

crates:
  - name: montrs-core
    bump: major
    note: "Breaking: removed deprecated `old_function`"
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
montrs agent prdoc validate
```

Checks:
- Title is not `...`
- At least one doc section with non-`...` description
- At least one crate listed
- Schema compliance

### 7. Verify "Removed" vs "Moved"

The generator detects moves automatically. Verify:

- Items moved between files appear correctly
- Bump level is not `major` for pure moves
- Actual removals have `major` bump

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

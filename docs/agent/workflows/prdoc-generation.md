# PRDoc Generation Workflow

This workflow ensures accurate classification of changes when generating PRDoc files.

## Steps

### 1. Generate Initial PRDoc

```bash
montrs agent prdoc generate --pr <number>
```

Or from a local diff:

```bash
montrs agent prdoc generate --from-diff changes.diff
```

### 2. CRITICAL: Verify "Removed" vs "Moved" Classification

Before accepting the generated PRDoc, **always verify** that items listed as "Removed" are truly removed and not moved.

#### How to Check

1. **Cross-reference every `-pub` item with `+pub` items**:
   - Look for matching names across removed and added items
   - Same name + same type (fn/struct/enum/trait) + different file path = **MOVE**, not removal

2. **Check the "Moved" section** (if present):
   - The generator now detects moves automatically
   - Verify items in "Moved" section have correct `from_path` → `to_path`

3. **Review path changes**:
   - Files renamed: `foo.rs` → `bar.rs` with same public items = move
   - Directory restructure: `src/a/mod.rs` → `src/b/mod.rs` = move

#### Manual Verification Checklist

- [ ] Every item in "Removed" section is truly deleted (no corresponding addition)
- [ ] Items that appear in both additions and removals are in "Moved" section
- [ ] Breaking change flag is NOT set for pure moves
- [ ] Version bump is `minor` (not `major`) for moves without actual removals

### 3. Review Generated Summary Sections

Verify the generated `prdoc.md` has accurate sections:

| Section | Should Contain |
|---------|---------------|
| **Added** | New public API items (not present before) |
| **Moved** | Relocated items with old → new paths |
| **Removed** | Deleted items (not moved elsewhere) |

### 4. Validate PRDoc

```bash
montrs agent prdoc validate
```

### 5. Run Agent Check

```bash
montrs agent check
```

## Common Scenarios

### File Rename with Same Public API

```diff
- packages/core/src/old.rs
+ packages/core/src/new.rs
```

- Items like `pub fn foo()` appear as removed from `old.rs` and added to `new.rs`
- Should appear in **Moved** section, NOT Removed
- No breaking change, bump = `minor` or `patch`

### Extract Module to Separate File

```diff
- packages/core/src/lib.rs  (-pub mod foo)
+ packages/core/src/foo/mod.rs  (+pub mod foo)
```

- Module `foo` moved, not removed
- Bump = `patch` (internal restructure)

### Actual Removal

```diff
- packages/core/src/deprecated.rs
- pub fn old_function()
```

- No corresponding `+pub fn old_function()` anywhere
- Should appear in **Removed** section
- Breaking change = `true`, bump = `major`

## Failure Recovery

If the generator misclassifies a move as removal:

1. Manually edit `prdoc.md`
2. Move item from "Removed" to "Moved" section
3. Add `from_path` and `to_path` details
4. Change `breaking: true` to `breaking: false` (if no actual removals)
5. Adjust crate `bump` from `major` to appropriate level

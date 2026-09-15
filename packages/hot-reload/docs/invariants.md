# Hot-Reload Package Invariants

## 1. Responsibility
`montrs-hot-reload` provides `view!` hot reload: it parses `view!` macros out of
Rust source, diffs them against a baseline, and produces markup patches that the
dev overlay applies to the live DOM without a rebuild.

## 2. Invariants
- **Pure analysis**: parsing and diffing only. No DOM access, no file watching,
  no build orchestration.
- **Conservative classification**: a change is reported as `view`-only only when
  it is certainly confined to markup; otherwise the caller must fall back to a
  full rebuild.
- **Deterministic output**: patch ordering and ids are stable for identical
  input, so the client can apply them idempotently.
- **Workspace-relative ids**: patch ids use workspace-relative paths to match the
  ids the `leptos` macro emits.

## 3. Boundary Definitions
- **In-Scope**: `view!` extraction, skeleton classification, patch generation.
- **Out-of-Scope**: CSS/assets, Rust logic changes, transport to the browser.

## 4. Agent Guidelines
- `ViewPatcher` is the public surface; `patch(path)` returns `None` for a
  non-view change and `is_view_only(path)` decides whether a rebuild is needed.

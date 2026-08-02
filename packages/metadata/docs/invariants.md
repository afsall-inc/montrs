# montrs-metadata Invariants

## What It Enforces
- All project metadata lives in `montrs.toml` — no `[[workspace.metadata.leptos]]` needed
- Auto-detects `bin-package` and `lib-package` from Cargo workspace if not specified
- Auto-detects project name from `Cargo.toml` if not specified
- Generates leptos-compatible metadata section on the fly

## Rules
- Always read from `montrs.toml` first, fall back to defaults
- Never modify the user's `Cargo.toml` unless explicitly requested
- The `to_leptos_metadata_section()` output must be 100% compatible with cargo-leptos
- All fields must have sensible defaults

## Boundary
- **In-Scope**: Reading `montrs.toml`, generating leptos metadata, auto-detection
- **Out-of-Scope**: Building, serving, or compiling — those are handled by the CLI
# montrs-registry — Invariants

## 1. Responsibility
Provide tool metadata (backends, bins, descriptions) for MontRS version management.

## 2. Invariants
- **Baked at compile time**: The base registry is embedded via `include_str!` for zero network overhead.
- **Floating fallback**: A downloadable registry from `montrs.com` can override the baked one.
- **Platform-aware**: Backend selection considers the current OS via `platform` overrides.
- **TOML format**: Registry files are TOML with `name`, `backends`, `bins`, `aliases`, `platform`.

## 3. Boundary
- **In-Scope**: Tool metadata, search, backend selection, registry caching.
- **Out-of-Scope**: Tool installation, version resolution, binary management.

## 4. Agent Guidelines
- Use `BAKED_REGISTRY` for compile-time access.
- Use `load_registry_from_dir()` for custom registry directories.
- Use `search()` for fuzzy matching against tool names/descriptions/bins.
# montrs-deps — Invariants

## 1. Responsibility
Manage project dependencies declared in the `[deps]` section of `montrs.toml`. Check source freshness against outputs to determine if a dependency needs reinstallation.

## 2. Invariants
- **Provider pattern**: Dependencies are `provider:target` (e.g., `cargo:ripgrep@14.0.0`).
- **Freshness over accuracy**: The freshness check is a heuristic (source hash comparison). It is not a cryptographic guarantee.
- **No auto-install**: This package only checks freshness. Actual installation is delegated to the appropriate tool.
- **State in `.montrs/deps/`**: Source hashes are persisted per-project.

## 3. Boundary
- **In-Scope**: DepSpec parsing, freshness checking, source hashing, state persistence.
- **Out-of-Scope**: Package installation, version resolution, lockfile generation.

## 4. Agent Guidelines
- Use `DepsManager::new()` then `load_from_config()` to load deps.
- Use `check_freshness()` to determine if a dep needs rebuilding.
- Use `known_lockfiles()` to check for common lockfile patterns.
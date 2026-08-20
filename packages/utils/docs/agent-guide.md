# Agent Guide: montrs-utils

## Core Concepts
Generic pure functions and shared utilities used across the MontRS workspace.

### Available Utilities
- **String helpers** — case conversion, trimming, slug generation.
- **Path helpers** — safe path joining, extension checking.
- **Collection helpers** — map, filter, group operations on Vec/slices.
- **Type helpers** — `Cow` optimizations, `Option` combinators.
- **Testing helpers** — deterministic ID generation, test fixtures.

### Usage
- Utilities are re-exported through the `montrs` facade.
- Use `montrs::utils::*` or `use montrs_utils::*` for direct access.
- Functions are pure, deterministic, and well-tested.

## Important Rules
- All functions are pure — no side effects, no I/O.
- Functions are generic where it improves reusability.
- No MontRS package dependencies — only stdlib.
- Tests are required for all public functions.
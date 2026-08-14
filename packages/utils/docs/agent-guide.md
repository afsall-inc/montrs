# Utils Package — Agent Guide

## Overview
`montrs-utils` provides generic pure functions used across MontRS packages. Includes string manipulation, path utilities, and other helpers.

## Key Concepts
- **to_pascal_case**: Converts `snake_case` to `PascalCase`.
- **to_kebab_case**: Converts `snake_case` to `kebab-case`.
- **Pure Functions**: All functions are deterministic and side-effect-free.

## Agent Usage
- Use for text/string manipulation across the framework.
- All functions are pure — no IO or state.

## Local Invariants
Read `docs/invariants.md` before modifying.
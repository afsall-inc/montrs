# montrs-utils

Generic pure functions shared across MontRS packages.

**Target Audiences:** Framework Contributors.

## 1. What this package is
`montrs-utils` provides small, dependency-free helpers (case conversion, string utilities, formatting) used throughout the framework.

## 2. What problems it solves
- **Duplication**: One home for shared pure functions.
- **Consistency**: Case conversion helpers that match MontRS conventions (pascal, snake, kebab).

## 3. What it intentionally does NOT do
- **Domain logic**: No auth, build, or rendering logic.
- **I/O**: Pure functions only.

## 4. How it fits into the MontRS system
Layer 1 dependency used by many packages.

## 5. When a user should reach for this package
- Needing case conversion or string helpers in a MontRS package.

## 6. Deeper Documentation
- [Invariants](docs/invariants.md)
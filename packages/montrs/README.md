# montrs

The MontRS facade crate — a single import surface for the framework.

**Target Audiences:** Application Developers, Agents.

## 1. What this package is
`montrs` re-exports the public API of MontRS's foundational packages so applications can depend on one crate. It contains minimal logic by design.

## 2. What problems it solves
- **Import friction**: One `use montrs::*` instead of a dozen package imports.
- **Feature surface**: Cargo features gate which packages are included.

## 3. What it intentionally does NOT do
- **Logic**: All real logic lives in the underlying packages.
- **CLI**: The `montrs` binary lives in `montrs-cli`.

## 4. How it fits into the MontRS system
Layer 3 facade, re-exporting `montrs-core`, `montrs-platform`, `montrs-ui`, and friends behind feature flags.

## 5. When a user should reach for this package
- Building an application and wanting a single dependency.

## 6. Deeper Documentation
- [Invariants](docs/invariants.md)
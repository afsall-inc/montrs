# PRDoc Package Invariants

## 1. Responsibility
`montrs-prdoc` provides PR documentation parsing, generation, and changelog management for MontRS.

## 2. Invariants
- **Structured Format**: PR docs follow a structured TOML schema with `title`, `audience`, `changes`, `impact` sections.
- **Audience Types**: Must support `FrameworkDev`, `AppDev`, `Operator` audiences.
- **Changelog Generation**: Must be able to generate changelogs from multiple PR docs.

## 3. Boundary Definitions
- **In-Scope**: PR doc parsing, validation, changelog generation, schema definitions.
- **Out-of-Scope**: Git operations, CI integration, release management.

## 4. Agent Guidelines
- Use `parse_prdoc()` to parse a PR doc file.
- Validate with `validate()` before generation.
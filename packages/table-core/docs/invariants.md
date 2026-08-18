# Table Core Package Invariants

## Responsibility
`montrs-table-core` provides headless, platform-independent table state and row transformations.

## Invariants
- No Leptos, DOM, JavaScript, or renderer dependencies.
- Row selection uses stable row IDs, never array indexes as business identity.
- Table state is serializable and suitable for server-side/manual mode.
- Transformations are explicit and deterministic.
- Renderers belong in UI, TUI, or native packages.

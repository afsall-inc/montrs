# Command Package Invariants

## Responsibility
`montrs-command` provides a framework-independent command registry and deterministic prefix search for command palettes and other interfaces.

## Invariants
- Command IDs are stable and unique within a registry.
- Registration replaces an existing command with the same ID.
- Search is deterministic and case-insensitive.
- Callbacks and rendering belong to consuming UI/platform adapters.

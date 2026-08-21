# Hotkeys Core Package Invariants

## Responsibility
`montrs-hotkeys-core` parses and matches keyboard shortcuts without depending on a UI framework or platform event system.

## Invariants
- No Leptos, DOM, web-sys, TUI, or desktop dependencies.
- Parsing is deterministic and invalid input returns an error.
- Matching is explicit about modifiers.
- Platform adapters belong in separate packages.
- Shortcut strings are stable and serializable.

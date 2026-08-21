# Hotkeys Web Package Invariants

## Responsibility
`montrs-hotkeys-web` adapts the platform-independent hotkey model to browser keyboard events.

## Invariants
- Browser APIs are isolated to this adapter.
- The core parser and matching semantics come from `montrs-hotkeys-core`.
- Event listeners must be cleaned up with the owning application/component lifecycle.
- Non-WASM builds remain compile-safe and do not perform browser work.

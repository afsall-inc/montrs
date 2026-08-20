# Agent Guide: montrs-hotkeys-web

## Core Concepts
Browser hotkey adapter with document-level listeners, scopes, and macros.

### HotkeysContext
- `keys_pressed: RwSignal<KeyPresses>` — currently pressed keys.
- `active_scopes: RwSignal<HashSet<String>>` — active scope names.
- `enable_scope`, `disable_scope`, `toggle_scope` — callbacks.

### Key Functions
- `provide_hotkeys_context(allow_blur, scopes)` — registers document keydown/keyup/blur listeners (WASM).
- `use_hotkeys_context()` — retrieves the context.
- `use_hotkeys_scoped(key, callback, scopes)` — register a scope-gated hotkey.
- `use_hotkeys_ref(node_ref, key, callback, scopes)` — element-scoped hotkey.

### Macros
- `scopes!()` — creates a HashSet with `"*"` as default scope.
- `use_hotkeys!(("meta+k") => callback)` — shorthand for `use_hotkeys_scoped`.
- `use_hotkeys!((key, "scope1", "scope2") => callback)` — with explicit scopes.
- `use_hotkeys_ref!((ref, key) => callback)` — element-scoped hotkey.

## Important Rules
- WASM-specific code is gated behind `#[cfg(target_arch = "wasm32")]`.
- Non-WASM builds are compile-safe no-ops.
- Scopes always include `"*"` by default.
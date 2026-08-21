# Agent Guide: montrs-hotkeys-core

## Core Concepts
Platform-independent keyboard shortcut parsing and matching.

### Hotkey
- `Hotkey::new("ctrl+shift+k")` — parse a key combination string.
- `FromStr` impl — `"ctrl+k".parse::<Hotkey>()`.
- `matches(&KeyEvent)` — check if a key event matches.
- `modifiers()` — returns `KeyboardModifiers`.
- `keys()` — returns the non-modifier keys.

### KeyboardModifiers
- Fields: `alt`, `ctrl`, `meta`, `shift` (all `bool`).
- `Display` — formats as `"Ctrl+Alt"`.
- Supports aliases: `ctrl`/`control`, `alt`/`option`, `meta`/`cmd`/`command`/`super`/`win`/`mod`.

### KeyPresses
- Tracks pressed keys via `key_map` (BTreeMap) and `last_key`.
- `push()`, `release()`, `clear()`.

### Helper Functions
- `is_last_key_match(parsed, pressed)` — checks if the last pressed key is the hotkey's final key.
- `is_hotkey_match(hotkey, pressed)` — full modifier + key check.

## Important Rules
- No Leptos, DOM, or event-system dependency.
- Parsing is deterministic.
- Matching is explicit about modifier sets.
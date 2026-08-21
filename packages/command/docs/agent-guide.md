# Agent Guide: montrs-command

## Core Concepts
Typed command registry for command palettes and keyboard-driven interfaces.

### Command
- `Command::new(id, name)` — create a command with a stable ID.
- `keywords` — alternative search terms for the command.
- `shortcut` — optional keyboard shortcut string.

### CommandRegistry
- `register()` — register or replace a command by ID.
- `search(query)` — case-insensitive prefix search across names and keywords.
- Deterministic ordering by ID.

## Important Rules
- Command IDs are stable and unique.
- Registration replaces an existing command with the same ID.
- Search is deterministic and case-insensitive.
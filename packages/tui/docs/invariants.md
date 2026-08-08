# montrs-tui — Invariants

## 1. Responsibility
Provide a terminal UI library for MontRS: buffer management, rendering, event handling, and composable widgets.

## 2. Invariants
- **No backend dependency**: Pure Rust ANSI output — no ncurses, termion, crossterm.
- **Diff-based rendering**: Only changed cells are written to the terminal.
- **Event thread**: Input events are read on a background thread and sent via channels.
- **PlatformAdapter**: `TuiAdapter` implements `PlatformAdapter` for `Target::Tui`.

## 3. Boundary
- **In-Scope**: Buffer, terminal I/O, renderer, events, renderables, text system, keymap, vnode.
- **Out-of-Scope**: Application framework, routing, state management, web rendering.

## 4. Agent Guidelines
- Create a `Buffer` with the desired size, then render widgets into it.
- Use `CliRenderer::render()` to display the buffer with diff-based updates.
- Use `EventBus` to receive keyboard and mouse events.
- Use `TuiAdapter` for `Target::Tui` platform integration.
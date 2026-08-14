# montrs-tui — Agent Guide

## Overview
Terminal UI library for MontRS. Provides the building blocks for interactive terminal applications.

## Key Concepts
- **Buffer**: 2D grid of `Cell` objects with color and attribute support.
- **CliRenderer**: Renders buffers to the terminal with diff-based output.
- **EventBus**: Receives keyboard, mouse, and terminal events.
- **Renderable trait**: Anything that can render into a buffer.
- **TuiAdapter**: Platform adapter for `Target::Tui`.

## Agent Usage
- Create a `Buffer::new(width, height)` to draw on.
- Use renderables (e.g., `BoxRenderable`, `TextRenderable`) to draw content.
- Call `renderer.render(&buffer)` to display.
- Use `TuiAdapter::new()` for platform integration.

## Local Invariants
Read `docs/invariants.md` before modifying.
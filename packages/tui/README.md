# montrs-tui

Terminal UI library for MontRS.

## Features

- **Buffer system**: 2D cell buffer with ANSI color/attribute support
- **Terminal I/O**: Raw mode, mouse support, bracketed paste, color detection
- **Renderer**: Diff-based rendering with full-screen and inline modes
- **Event system**: Keyboard, mouse, paste, resize events with background thread
- **21 Renderables**: Box, Text, Scrollbox, Select, Table, Input, Markdown, Diff, Code, Image, LineNumber, Slider, ScrollBar, TabSelect, FrameBuffer, AsciiFont, Editor, Textarea
- **Text system**: Rope, EditBuffer, StyledText, StyledChunk
- **Animation**: Timeline, Easing
- **Keymap**: Keybinding system
- **VNode**: Virtual node composition (hyperscript-style)
- **TuiAdapter**: `PlatformAdapter` for `Target::Tui`

## Usage

```rust
use montrs_tui::buffer::Buffer;
use montrs_tui::renderables::box_renderable::BoxRenderable;
use montrs_tui::renderables::Renderable;

let mut buffer = Buffer::new(80, 24);
let bx = BoxRenderable::new().with_title("Hello");
bx.render(&mut buffer, 0, 0, 80, 24);
println!("{}", buffer.to_ansi());
```
/// Terminal renderer — draws buffers to the screen.
use crate::buffer::Buffer;
use std::io::{self, Write};

/// Screen mode for the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenMode {
    /// Full-screen alternate buffer.
    FullScreen,
    /// Inline within the terminal.
    Inline,
}

/// Renderer configuration.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub mode: ScreenMode,
    pub show_cursor: bool,
    pub cursor_position: Option<(usize, usize)>,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            mode: ScreenMode::FullScreen,
            show_cursor: false,
            cursor_position: None,
        }
    }
}

/// A renderer that draws buffers to the terminal.
pub struct CliRenderer {
    pub config: RenderConfig,
    pub last_buffer: Option<Buffer>,
}

impl Default for CliRenderer {
    fn default() -> Self {
        Self::new(RenderConfig::default())
    }
}

impl CliRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self {
            config,
            last_buffer: None,
        }
    }

    /// Render a buffer, only outputting changed cells (diff-based).
    pub fn render(&mut self, buffer: &Buffer) -> io::Result<()> {
        let mut out = String::new();

        // Move cursor to home.
        out.push_str("\x1b[H");

        match &self.last_buffer {
            Some(prev) => {
                // Diff: only write cells that changed.
                for y in 0..buffer.height {
                    for x in 0..buffer.width {
                        let current = buffer.cell(x, y);
                        let old = prev.cell(x, y);
                        if current != old {
                            out.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
                            out.push_str(&current.fg.ansi_code());
                            out.push_str(&current.bg.ansi_bg());
                            out.push(current.ch);
                        }
                    }
                }
            }
            None => {
                // Full render.
                out.push_str(&buffer.to_ansi());
            }
        }

        if let Some((x, y)) = self.config.cursor_position {
            out.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
        }
        if self.config.show_cursor {
            out.push_str("\x1b[?25h");
        } else {
            out.push_str("\x1b[?25l");
        }

        io::stdout().write_all(out.as_bytes())?;
        io::stdout().flush()?;
        self.last_buffer = Some(buffer.clone());
        Ok(())
    }

    /// Clear the screen.
    pub fn clear_screen(&self) -> io::Result<()> {
        io::stdout().write_all(b"\x1b[2J\x1b[H")?;
        io::stdout().flush()?;
        Ok(())
    }
}

/// A scrollback surface (for inline mode).
#[derive(Debug, Clone)]
pub struct ScrollbackSurface {
    pub lines: Vec<String>,
    pub max_lines: usize,
}

impl Default for ScrollbackSurface {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            max_lines: 1000,
        }
    }
}

impl ScrollbackSurface {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: Vec::new(),
            max_lines,
        }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
        if self.lines.len() > self.max_lines {
            let overflow = self.lines.len() - self.max_lines;
            self.lines.drain(0..overflow);
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

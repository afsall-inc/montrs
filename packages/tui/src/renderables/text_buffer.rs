/// Text buffer renderable — scrollable text display.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct TextBufferRenderable {
    pub lines: Vec<String>,
    pub scroll_offset: usize,
    pub fg: Color,
    pub bg: Color,
}

impl TextBufferRenderable {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            scroll_offset: 0,
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
    pub fn with_lines(mut self, lines: Vec<String>) -> Self {
        self.lines = lines;
        self
    }
    pub fn scroll_to(&mut self, offset: usize) {
        self.scroll_offset = offset;
    }
}

impl Default for TextBufferRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TextBufferRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        for row in 0..height {
            let line_idx = self.scroll_offset + row;
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let max_chars = width.min(line.len());
            for (i, c) in line.chars().enumerate().take(max_chars) {
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, self.fg, self.bg, CharAttribute::default()),
                );
            }
        }
    }
}

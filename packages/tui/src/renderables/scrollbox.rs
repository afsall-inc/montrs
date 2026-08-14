/// Scrollbox renderable — scrollable viewport with content.
use crate::buffer::{Buffer, Cell};
use crate::renderables::Renderable;

pub struct ScrollBoxRenderable {
    pub content: Vec<String>,
    pub scroll_x: usize,
    pub scroll_y: usize,
}

impl ScrollBoxRenderable {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            scroll_x: 0,
            scroll_y: 0,
        }
    }
    pub fn with_content(mut self, content: Vec<String>) -> Self {
        self.content = content;
        self
    }
}

impl Default for ScrollBoxRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ScrollBoxRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        for row in 0..height {
            let line_idx = self.scroll_y + row;
            if line_idx >= self.content.len() {
                break;
            }
            let line = &self.content[line_idx];
            let start = self.scroll_x.min(line.len());
            let end = (start + width).min(line.len());
            for (i, c) in line[start..end].chars().enumerate() {
                buffer.set(x + i, y + row, Cell::new(c));
            }
        }
    }
}

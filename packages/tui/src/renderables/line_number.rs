/// LineNumber renderable — line number gutter.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct LineNumberRenderable {
    pub line_count: usize,
    pub scroll_offset: usize,
    pub active_line: Option<usize>,
}

impl LineNumberRenderable {
    pub fn new() -> Self {
        Self {
            line_count: 0,
            scroll_offset: 0,
            active_line: None,
        }
    }
    pub fn with_line_count(mut self, count: usize) -> Self {
        self.line_count = count;
        self
    }
}

impl Default for LineNumberRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for LineNumberRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        let width = width.clamp(3, 6);
        for row in 0..height {
            let line_num = self.scroll_offset + row + 1;
            if line_num > self.line_count {
                break;
            }
            let is_active = self.active_line == Some(line_num - 1);
            let fg = if is_active {
                Color::Cyan
            } else {
                Color::Rgb(100, 100, 100)
            };
            let num_str = format!("{:>width$}", line_num, width = width);
            for (i, c) in num_str.chars().enumerate() {
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, Color::Reset, CharAttribute::default()),
                );
            }
        }
    }
}

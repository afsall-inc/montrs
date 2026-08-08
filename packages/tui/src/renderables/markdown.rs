/// Markdown renderable — renders markdown text with basic formatting.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct MarkdownRenderable {
    pub text: String,
}

impl MarkdownRenderable {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Render markdown to pre-formatted lines with basic style info.
    pub fn render_lines(&self) -> Vec<(String, bool)> {
        self.text
            .lines()
            .map(|line| {
                if line.starts_with("#") {
                    (line.trim_start_matches('#').trim().to_string(), true)
                } else {
                    (line.to_string(), false)
                }
            })
            .collect()
    }
}

impl Default for MarkdownRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for MarkdownRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        let lines = self.render_lines();
        for row in 0..height {
            if row >= lines.len() {
                break;
            }
            let (line, is_heading) = &lines[row];
            let fg = if *is_heading {
                Color::Cyan
            } else {
                Color::Reset
            };
            let attr = CharAttribute {
                bold: *is_heading,
                ..Default::default()
            };
            let max_chars = width.min(line.len());
            for (i, c) in line.chars().enumerate().take(max_chars) {
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, Color::Reset, attr),
                );
            }
        }
    }
}

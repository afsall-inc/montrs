/// Code renderable — syntax-highlighted code display.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct CodeRenderable {
    pub lines: Vec<String>,
    pub language: Option<String>,
    pub scroll_offset: usize,
}

impl CodeRenderable {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            language: None,
            scroll_offset: 0,
        }
    }
    pub fn with_code(mut self, code: &str) -> Self {
        self.lines = code.lines().map(|s| s.to_string()).collect();
        self
    }
    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_string());
        self
    }
}

impl Default for CodeRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for CodeRenderable {
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
                let fg = simple_highlight(c, &self.language);
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, Color::Reset, CharAttribute::default()),
                );
            }
        }
    }
}

fn simple_highlight(c: char, _lang: &Option<String>) -> Color {
    match c {
        '#' | ';' | '/' => Color::Rgb(120, 120, 120), // comments
        '"' | '\'' => Color::Rgb(210, 180, 140),      // strings
        '0'..='9' => Color::Rgb(220, 220, 100),       // numbers
        _ => Color::Reset,
    }
}

/// TabSelect renderable — horizontal tab bar.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct TabSelectRenderable {
    pub tabs: Vec<String>,
    pub selected: usize,
}

impl TabSelectRenderable {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selected: 0,
        }
    }
    pub fn with_tabs(mut self, tabs: Vec<String>) -> Self {
        self.tabs = tabs;
        self
    }
    pub fn select(&mut self, idx: usize) {
        self.selected = idx.min(self.tabs.len().saturating_sub(1));
    }
    pub fn next(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = (self.selected + 1) % self.tabs.len();
        }
    }
    pub fn prev(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = self.selected.saturating_sub(1);
        }
    }
}

impl Default for TabSelectRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TabSelectRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
    ) {
        let mut cx = x;
        for (idx, tab) in self.tabs.iter().enumerate() {
            let is_selected = idx == self.selected;
            let text = format!(" {} ", tab);
            let fg = if is_selected {
                Color::Black
            } else {
                Color::Reset
            };
            let bg = if is_selected {
                Color::Cyan
            } else {
                Color::Reset
            };
            for (i, c) in text.chars().enumerate() {
                buffer.set(
                    cx + i,
                    y,
                    Cell::styled(c, fg, bg, CharAttribute::default()),
                );
            }
            cx += text.len();
            // Separator between tabs
            if idx < self.tabs.len() - 1 {
                buffer.set(
                    cx,
                    y,
                    Cell::styled(
                        '│',
                        Color::Rgb(100, 100, 100),
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
                cx += 1;
            }
        }
    }
}

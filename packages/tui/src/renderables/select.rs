/// Select renderable — list selection widget.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct SelectRenderable {
    pub items: Vec<String>,
    pub selected: usize,
}

impl SelectRenderable {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
        }
    }
    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }
    pub fn select(&mut self, idx: usize) {
        self.selected = idx.min(self.items.len().saturating_sub(1));
    }
}

impl Default for SelectRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for SelectRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        for row in 0..height {
            if row >= self.items.len() {
                break;
            }
            let item = &self.items[row];
            let is_selected = row == self.selected;
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
            let marker = if is_selected { ">" } else { " " };
            // Write selection marker
            buffer.set(
                x,
                y + row,
                Cell::styled(
                    marker.chars().next().unwrap(),
                    fg,
                    bg,
                    CharAttribute::default(),
                ),
            );
            // Write item text
            let max_chars = width.saturating_sub(2).min(item.len());
            for (i, c) in item.chars().enumerate().take(max_chars) {
                buffer.set(
                    x + 2 + i,
                    y + row,
                    Cell::styled(c, fg, bg, CharAttribute::default()),
                );
            }
        }
    }
}

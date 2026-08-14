/// Input renderable — single-line text input.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct InputRenderable {
    pub value: String,
    pub cursor_pos: usize,
    pub placeholder: String,
}

impl InputRenderable {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor_pos: 0,
            placeholder: String::new(),
        }
    }
    pub fn with_value(mut self, val: &str) -> Self {
        self.value = val.to_string();
        self.cursor_pos = val.len();
        self
    }
    pub fn with_placeholder(mut self, ph: &str) -> Self {
        self.placeholder = ph.to_string();
        self
    }
    pub fn insert(&mut self, c: char) {
        self.value.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }
    pub fn delete(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.value.remove(self.cursor_pos);
        }
    }
    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.value.remove(self.cursor_pos);
        }
    }
    pub fn move_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }
    pub fn move_right(&mut self) {
        self.cursor_pos = self.cursor_pos.min(self.value.len());
    }
}

impl Default for InputRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for InputRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        _height: usize,
    ) {
        let display = if self.value.is_empty() && !self.placeholder.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };
        let fg = if self.value.is_empty() && !self.placeholder.is_empty() {
            Color::Rgb(100, 100, 100)
        } else {
            Color::Reset
        };
        let max_chars = width.min(display.len());
        for (i, c) in display.chars().enumerate().take(max_chars) {
            buffer.set(
                x + i,
                y,
                Cell::styled(c, fg, Color::Reset, CharAttribute::default()),
            );
        }
        // Cursor
        if self.cursor_pos < width {
            let cursor_char =
                display.chars().nth(self.cursor_pos).unwrap_or(' ');
            buffer.set(
                x + self.cursor_pos,
                y,
                Cell::styled(
                    cursor_char,
                    Color::Black,
                    Color::White,
                    CharAttribute::default(),
                ),
            );
        }
    }
}

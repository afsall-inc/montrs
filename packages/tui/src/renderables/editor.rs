/// Editor renderable — multi-line editor with cursor.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct EditorRenderable {
    pub lines: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
}

impl EditorRenderable {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }
    pub fn with_text(mut self, text: &str) -> Self {
        self.lines = text.lines().map(|s| s.to_string()).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self
    }
    pub fn insert_char(&mut self, c: char) {
        if self.cursor_line >= self.lines.len() {
            return;
        }
        self.lines[self.cursor_line].insert(self.cursor_col, c);
        self.cursor_col += 1;
    }
    pub fn newline(&mut self) {
        let line = self.lines[self.cursor_line].clone();
        let (left, right) = line.split_at(self.cursor_col);
        self.lines[self.cursor_line] = left.to_string();
        self.lines.insert(self.cursor_line + 1, right.to_string());
        self.cursor_line += 1;
        self.cursor_col = 0;
    }
    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.lines[self.cursor_line].remove(self.cursor_col - 1);
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            let prev_len = self.lines[self.cursor_line - 1].len();
            let cur = self.lines.remove(self.cursor_line);
            self.lines[self.cursor_line - 1].push_str(&cur);
            self.cursor_line -= 1;
            self.cursor_col = prev_len;
        }
    }
}

impl Default for EditorRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for EditorRenderable {
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
                buffer.set(x + i, y + row, Cell::new(c));
            }
            // Cursor
            if line_idx == self.cursor_line
                && (self.cursor_col as i32 - self.scroll_offset as i32) >= 0
            {
                let cursor_x = self.cursor_col;
                if cursor_x < width {
                    let ch = line.chars().nth(cursor_x).unwrap_or(' ');
                    buffer.set(
                        x + cursor_x,
                        y + row,
                        Cell::styled(
                            ch,
                            Color::Black,
                            Color::White,
                            CharAttribute::default(),
                        ),
                    );
                }
            }
        }
    }
}

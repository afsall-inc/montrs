/// Diff renderable — unified/split diff display.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMode {
    Unified,
    Split,
}

pub struct DiffRenderable {
    pub lines: Vec<DiffLine>,
    pub mode: DiffMode,
    pub scroll_offset: usize,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub text: String,
    pub kind: DiffLineKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Add,
    Remove,
    Context,
    Header,
}

impl DiffRenderable {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            mode: DiffMode::Unified,
            scroll_offset: 0,
        }
    }
    pub fn parse_unified(mut self, diff_text: &str) -> Self {
        for line in diff_text.lines() {
            let kind = if line.starts_with("+") {
                DiffLineKind::Add
            } else if line.starts_with("-") {
                DiffLineKind::Remove
            } else if line.starts_with("@@") {
                DiffLineKind::Header
            } else {
                DiffLineKind::Context
            };
            self.lines.push(DiffLine {
                text: line.to_string(),
                kind,
            });
        }
        self
    }
}

impl Default for DiffRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for DiffRenderable {
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
            let (fg, bg) = match line.kind {
                DiffLineKind::Add => {
                    (Color::Rgb(80, 200, 80), Color::Rgb(20, 60, 20))
                }
                DiffLineKind::Remove => {
                    (Color::Rgb(200, 80, 80), Color::Rgb(60, 20, 20))
                }
                DiffLineKind::Header => (Color::Cyan, Color::Reset),
                DiffLineKind::Context => (Color::Reset, Color::Reset),
            };
            let max_chars = width.min(line.text.len());
            for (i, c) in line.text.chars().enumerate().take(max_chars) {
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, bg, CharAttribute::default()),
                );
            }
        }
    }
}

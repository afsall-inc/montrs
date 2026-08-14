/// Box renderable — a bordered box with optional title.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct BoxRenderable {
    pub title: Option<String>,
    pub border_style: BorderStyle,
    pub fill: Option<char>,
}

pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    None,
}

impl BoxRenderable {
    pub fn new() -> Self {
        Self {
            title: None,
            border_style: BorderStyle::Rounded,
            fill: None,
        }
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

impl Default for BoxRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for BoxRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        if width < 3 || height < 3 {
            return;
        }
        let (tl, t, tr, l, r, bl, b, br) = match self.border_style {
            BorderStyle::Single => ("┌", "─", "┐", "│", "│", "└", "─", "┘"),
            BorderStyle::Double => ("╔", "═", "╗", "║", "║", "╚", "═", "╝"),
            BorderStyle::Rounded => ("╭", "─", "╮", "│", "│", "╰", "─", "╯"),
            BorderStyle::None => return,
        };
        let border = Color::Rgb(100, 100, 100);
        // Top
        buffer.set(
            x,
            y,
            Cell::styled(
                tl.chars().next().unwrap(),
                border,
                Color::Reset,
                CharAttribute::default(),
            ),
        );
        for cx in (x + 1)..(x + width - 1) {
            buffer.set(
                cx,
                y,
                Cell::styled(
                    t.chars().next().unwrap(),
                    border,
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
        }
        buffer.set(
            x + width - 1,
            y,
            Cell::styled(
                tr.chars().next().unwrap(),
                border,
                Color::Reset,
                CharAttribute::default(),
            ),
        );
        // Sides
        for row in (y + 1)..(y + height - 1) {
            buffer.set(
                x,
                row,
                Cell::styled(
                    l.chars().next().unwrap(),
                    border,
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
            buffer.set(
                x + width - 1,
                row,
                Cell::styled(
                    r.chars().next().unwrap(),
                    border,
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
        }
        // Bottom
        buffer.set(
            x,
            y + height - 1,
            Cell::styled(
                bl.chars().next().unwrap(),
                border,
                Color::Reset,
                CharAttribute::default(),
            ),
        );
        for cx in (x + 1)..(x + width - 1) {
            buffer.set(
                cx,
                y + height - 1,
                Cell::styled(
                    b.chars().next().unwrap(),
                    border,
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
        }
        buffer.set(
            x + width - 1,
            y + height - 1,
            Cell::styled(
                br.chars().next().unwrap(),
                border,
                Color::Reset,
                CharAttribute::default(),
            ),
        );
        // Title
        if let Some(title) = &self.title {
            let title_max = width.saturating_sub(4);
            let display = if title.len() > title_max {
                format!("{}…", &title[..title_max.saturating_sub(1)])
            } else {
                title.clone()
            };
            let tx = x + 2;
            for (i, c) in display.chars().enumerate() {
                buffer.set(
                    tx + i,
                    y,
                    Cell::styled(
                        c,
                        Color::White,
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
            }
        }
    }
}

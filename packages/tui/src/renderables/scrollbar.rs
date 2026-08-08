/// ScrollBar renderable — arrows + slider.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct ScrollBarRenderable {
    pub position: f64,
    pub length: f64,
    pub vertical: bool,
}

impl ScrollBarRenderable {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            length: 0.1,
            vertical: true,
        }
    }
}

impl Default for ScrollBarRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ScrollBarRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        if self.vertical {
            // Up arrow
            buffer.set(
                x,
                y,
                Cell::styled(
                    '▲',
                    Color::Rgb(120, 120, 120),
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
            // Track
            let track_height = height.saturating_sub(2);
            let thumb_start =
                (self.position * track_height as f64).round() as usize;
            let thumb_end = ((self.position + self.length)
                * track_height as f64)
                .round() as usize;
            let thumb_end = thumb_end.min(track_height.saturating_sub(1));
            for i in 0..track_height {
                let ch = if i >= thumb_start && i <= thumb_end {
                    '█'
                } else {
                    '│'
                };
                let fg = if i >= thumb_start && i <= thumb_end {
                    Color::Cyan
                } else {
                    Color::Rgb(80, 80, 80)
                };
                buffer.set(
                    x,
                    y + 1 + i,
                    Cell::styled(
                        ch,
                        fg,
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
            }
            // Down arrow
            buffer.set(
                x,
                y + height - 1,
                Cell::styled(
                    '▼',
                    Color::Rgb(120, 120, 120),
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
        } else {
            // Horizontal scrollbar
            if width < 2 {
                return;
            }
            let thumb_start =
                (self.position * (width - 1) as f64).round() as usize;
            let thumb_end = ((self.position + self.length) * (width - 1) as f64)
                .round() as usize;
            let thumb_end = thumb_end.min(width - 1);
            for i in 0..width {
                let ch = if i == 0 {
                    '◄'
                } else if i == width - 1 {
                    '►'
                } else if i >= thumb_start && i <= thumb_end {
                    '█'
                } else {
                    '─'
                };
                let fg = if i >= thumb_start && i <= thumb_end {
                    Color::Cyan
                } else {
                    Color::Rgb(80, 80, 80)
                };
                buffer.set(
                    x + i,
                    y,
                    Cell::styled(
                        ch,
                        fg,
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
            }
        }
    }
}

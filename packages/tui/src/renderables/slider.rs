/// Slider renderable — scrollbar/slider indicator.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct SliderRenderable {
    pub position: f64, // 0.0 to 1.0
    pub length: f64,   // visible fraction (0.0 to 1.0)
}

impl SliderRenderable {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            length: 0.1,
        }
    }
    pub fn with_range(
        mut self,
        pos: usize,
        visible: usize,
        total: usize,
    ) -> Self {
        if total > 0 {
            self.position = pos as f64 / total as f64;
            self.length = (visible as f64 / total as f64).min(1.0);
        }
        self
    }
}

impl Default for SliderRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for SliderRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        _height: usize,
    ) {
        if width < 2 {
            return;
        }
        let thumb_start =
            (self.position * (width as f64 - 1.0)).round() as usize;
        let thumb_end = ((self.position + self.length) * (width as f64 - 1.0))
            .round() as usize;
        let thumb_end = thumb_end.min(width - 1);
        for i in 0..width {
            let ch = if i >= thumb_start && i <= thumb_end {
                '█'
            } else {
                '░'
            };
            let fg = if i >= thumb_start && i <= thumb_end {
                Color::Cyan
            } else {
                Color::Rgb(80, 80, 80)
            };
            buffer.set(
                x + i,
                y,
                Cell::styled(ch, fg, Color::Reset, CharAttribute::default()),
            );
        }
    }
}

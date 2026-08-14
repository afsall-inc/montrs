/// Image renderable — block-based image display (kitty/sixel/block).
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct ImageRenderable {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub mode: ImageMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageMode {
    Block,
    Kitty,
    Sixel,
}

impl ImageRenderable {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
            mode: ImageMode::Block,
        }
    }
    pub fn with_rgba(
        mut self,
        data: Vec<u8>,
        width: usize,
        height: usize,
    ) -> Self {
        self.data = data;
        self.width = width;
        self.height = height;
        self
    }

    /// Get the color of a pixel as block characters.
    fn pixel_color(&self, px: usize, py: usize) -> Color {
        if px >= self.width || py >= self.height {
            return Color::Reset;
        }
        let idx = (py * self.width + px) * 4;
        if idx + 2 >= self.data.len() {
            return Color::Reset;
        }
        Color::Rgb(self.data[idx], self.data[idx + 1], self.data[idx + 2])
    }
}

impl Default for ImageRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ImageRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        if self.data.is_empty() || self.width == 0 {
            return;
        }
        // Block mode: each terminal cell = 1 pixel (scaled down)
        let scale_x = (self.width as f64 / width as f64).max(1.0);
        let scale_y = (self.height as f64 / height as f64).max(1.0);
        for row in 0..height {
            for col in 0..width {
                let px = (col as f64 * scale_x) as usize;
                let py = (row as f64 * scale_y) as usize;
                let color = self.pixel_color(px, py);
                buffer.set(
                    x + col,
                    y + row,
                    Cell::styled(
                        '█',
                        color,
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
            }
        }
    }
}

/// Frame buffer renderable — wraps a frame buffer for display.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct FrameBufferRenderable {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl FrameBufferRenderable {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::Reset; width * height],
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
}

impl Renderable for FrameBufferRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
    ) {
        for row in 0..self.height {
            for col in 0..self.width {
                let color = self.pixels[row * self.width + col];
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

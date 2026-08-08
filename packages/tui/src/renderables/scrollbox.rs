use crate::{buffer::Buffer, renderables::Renderable};

pub struct ScrollboxRenderable;

impl ScrollboxRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScrollboxRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ScrollboxRenderable {
    fn render(
        &self,
        _buffer: &mut Buffer,
        _x: usize,
        _y: usize,
        _width: usize,
        _height: usize,
    ) {
    }
}

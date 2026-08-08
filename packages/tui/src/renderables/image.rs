use crate::{buffer::Buffer, renderables::Renderable};

pub struct ImageRenderable;

impl ImageRenderable {
    pub fn new() -> Self {
        Self
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
        _buffer: &mut Buffer,
        _x: usize,
        _y: usize,
        _width: usize,
        _height: usize,
    ) {
    }
}

use crate::{buffer::Buffer, renderables::Renderable};

pub struct TabSelectRenderable;

impl TabSelectRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TabSelectRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TabSelectRenderable {
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

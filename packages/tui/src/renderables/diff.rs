use crate::{buffer::Buffer, renderables::Renderable};

pub struct DiffRenderable;

impl DiffRenderable {
    pub fn new() -> Self {
        Self
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
        _buffer: &mut Buffer,
        _x: usize,
        _y: usize,
        _width: usize,
        _height: usize,
    ) {
    }
}

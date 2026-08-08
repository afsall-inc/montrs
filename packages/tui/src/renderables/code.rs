use crate::{buffer::Buffer, renderables::Renderable};

pub struct CodeRenderable;

impl CodeRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CodeRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for CodeRenderable {
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

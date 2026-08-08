use crate::{buffer::Buffer, renderables::Renderable};

pub struct AsciiFontRenderable;

impl AsciiFontRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AsciiFontRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for AsciiFontRenderable {
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

use crate::{buffer::Buffer, renderables::Renderable};

pub struct SelectRenderable;

impl SelectRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SelectRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for SelectRenderable {
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

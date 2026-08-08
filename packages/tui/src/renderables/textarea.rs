use crate::{buffer::Buffer, renderables::Renderable};

pub struct TextareaRenderable;

impl TextareaRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextareaRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TextareaRenderable {
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

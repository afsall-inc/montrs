use crate::{buffer::Buffer, renderables::Renderable};

pub struct MarkdownRenderable;

impl MarkdownRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MarkdownRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for MarkdownRenderable {
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

use crate::{buffer::Buffer, renderables::Renderable};

pub struct EditorRenderable;

impl EditorRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EditorRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for EditorRenderable {
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

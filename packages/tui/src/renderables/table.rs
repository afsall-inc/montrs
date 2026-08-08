use crate::{buffer::Buffer, renderables::Renderable};

pub struct TableRenderable;

impl TableRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TableRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TableRenderable {
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

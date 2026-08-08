use crate::{buffer::Buffer, renderables::Renderable};

pub struct LineNumberRenderable;

impl LineNumberRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LineNumberRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for LineNumberRenderable {
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

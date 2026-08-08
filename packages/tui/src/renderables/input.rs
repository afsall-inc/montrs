use crate::{buffer::Buffer, renderables::Renderable};

pub struct InputRenderable;

impl InputRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for InputRenderable {
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

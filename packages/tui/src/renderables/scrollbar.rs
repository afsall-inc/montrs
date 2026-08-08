use crate::{buffer::Buffer, renderables::Renderable};

pub struct ScrollbarRenderable;

impl ScrollbarRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScrollbarRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ScrollbarRenderable {
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

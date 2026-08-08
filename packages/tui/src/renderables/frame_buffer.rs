use crate::{buffer::Buffer, renderables::Renderable};

pub struct FrameBufferRenderable;

impl FrameBufferRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FrameBufferRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for FrameBufferRenderable {
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

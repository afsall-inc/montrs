use crate::{buffer::Buffer, renderables::Renderable};

pub struct SliderRenderable;

impl SliderRenderable {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SliderRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for SliderRenderable {
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

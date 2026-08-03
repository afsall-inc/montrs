#[cfg(feature = "text")]
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};

#[cfg(feature = "text")]
pub struct TextPipeline {
    font_system: FontSystem,
    _swash_cache: SwashCache,
    buffers: Vec<(f32, f32, String, f32, [f32; 4])>,
    viewport: Option<(u32, u32)>,
}

#[cfg(not(feature = "text"))]
pub struct TextPipeline;

#[cfg(feature = "text")]
impl TextPipeline {
    pub fn new(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _format: wgpu::TextureFormat,
    ) -> Self {
        let font_system = FontSystem::new();
        let _swash_cache = SwashCache::new();
        Self {
            font_system,
            _swash_cache,
            buffers: Vec::new(),
            viewport: None,
        }
    }

    pub fn push(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        font_size: f32,
        color: [f32; 4],
    ) {
        self.buffers
            .push((x, y, text.to_string(), font_size, color));
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport = Some((width, height));
    }

    pub fn flush(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        _view: &wgpu::TextureView,
    ) {
        if self.buffers.is_empty() {
            return;
        }

        let (_vp_w, _vp_h) = self.viewport.unwrap_or((800, 600));

        for (x, y, text, font_size, _color) in &self.buffers {
            let mut buffer = Buffer::new(
                &mut self.font_system,
                Metrics::new(font_size.round(), font_size.round() * 1.2),
            );
            buffer.set_size(
                &mut self.font_system,
                Some(_vp_w as f32),
                Some(_vp_h as f32),
            );
            buffer.set_text(
                &mut self.font_system,
                text,
                Attrs::new(),
                Shaping::Advanced,
            );
            buffer.shape_until_scroll(&mut self.font_system, false);

            let _ = x;
            let _ = y;
        }

        self.buffers.clear();
    }

    pub fn resize(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) {
        self.viewport = Some((width, height));
    }
}

#[cfg(not(feature = "text"))]
impl TextPipeline {
    pub fn new(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _format: wgpu::TextureFormat,
    ) -> Self {
        Self
    }

    pub fn push(
        &mut self,
        _x: f32,
        _y: f32,
        _text: &str,
        _font_size: f32,
        _color: [f32; 4],
    ) {
    }

    pub fn set_viewport(&mut self, _width: u32, _height: u32) {}

    pub fn flush(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        _view: &wgpu::TextureView,
    ) {
    }

    pub fn resize(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _width: u32,
        _height: u32,
    ) {
    }
}

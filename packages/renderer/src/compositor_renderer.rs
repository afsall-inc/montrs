//! CompositorRenderer — a Renderer implementation backed by the Compositor.
//!
//! Wraps a `Compositor` + a backend `Renderer` so that the compositor layer
//! stack is flushed to the backend on `finish()`. This is the primary way
//! to use the renderer in desktop and mobile apps.

use crate::*;

/// A Renderer implementation that buffers drawing commands through a
/// Compositor and flushes them to a backend Renderer on finish().
pub struct CompositorRenderer {
    compositor: Compositor,
    backend: Box<dyn Renderer>,
    current_viewport: Option<Viewport>,
}

impl CompositorRenderer {
    pub fn new(backend: Box<dyn Renderer>) -> Self {
        Self {
            compositor: Compositor::new(),
            backend,
            current_viewport: None,
        }
    }

    /// Returns a mutable reference to the compositor for direct layer manipulation.
    pub fn compositor(&mut self) -> &mut Compositor {
        &mut self.compositor
    }

    /// Returns a mutable reference to the backend renderer.
    pub fn backend(&mut self) -> &mut dyn Renderer {
        &mut *self.backend
    }

    /// Begin a new compositing layer with the given alpha.
    pub fn begin_layer(&mut self, alpha: f32) {
        self.compositor.begin_layer(alpha);
    }
}

impl Renderer for CompositorRenderer {
    fn begin(&mut self, viewport: &Viewport) {
        self.current_viewport = Some(*viewport);
        self.compositor = Compositor::new();
    }

    fn fill_quad(&mut self, quad: &Quad, paint: &Paint) {
        self.compositor.push_quad(quad.clone(), paint.clone());
    }

    fn fill_path(&mut self, path: &Path, paint: &Paint) {
        self.compositor.push_path(path.clone(), paint.clone());
    }

    fn stroke_path(&mut self, path: &Path, stroke: &Stroke, paint: &Paint) {
        self.compositor.push_stroke(path.clone(), stroke.clone(), paint.clone());
    }

    fn draw_glyphs(&mut self, _pos: Point, _glyphs: &[GlyphRun], _paint: &Paint) {
        // Compositor does not yet support text glyphs — will be added with text pipeline
    }

    fn draw_image(&mut self, image: &Image, rect: Rect) {
        self.compositor.push_image(image.clone(), rect);
    }

    fn draw_svg(&mut self, svg: &Svg, rect: Rect) {
        self.compositor.push_svg(svg.clone(), rect);
    }

    fn clip(&mut self, _shape: &Shape) {
        // Compositor clipping is pending implementation
    }

    fn clear_clip(&mut self) {
        // Compositor clipping is pending implementation
    }

    fn push_layer(&mut self, alpha: f32, _transform: &[f32; 6]) {
        self.compositor.begin_layer(alpha);
    }

    fn pop_layer(&mut self) {
        // Layers are flushed on finish()
    }

    fn finish(&mut self) -> Frame {
        if let Some(viewport) = &self.current_viewport {
            self.compositor.render(&mut *self.backend, viewport);
        }
        self.backend.finish()
    }
}
//! Layer-based compositing, inspired by iced's compositor.

use crate::{Paint, Quad, Renderer, Viewport};

/// A compositing layer containing quads, text, and images.
#[derive(Debug, Default)]
pub struct Layer {
    pub quads: Vec<Quad>,
    pub paints: Vec<Paint>,
    pub alpha: f32,
}

/// A compositor that manages layer stacking.
#[derive(Debug, Default)]
pub struct Compositor {
    layers: Vec<Layer>,
}

impl Compositor {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn begin_layer(&mut self, alpha: f32) {
        self.layers.push(Layer {
            alpha,
            ..Default::default()
        });
    }

    pub fn push_quad(&mut self, quad: Quad, paint: Paint) {
        if let Some(layer) = self.layers.last_mut() {
            layer.quads.push(quad);
            layer.paints.push(paint);
        }
    }

    /// Flatten all layers and render them using the given renderer.
    pub fn render(&mut self, renderer: &mut dyn Renderer, viewport: &Viewport) {
        renderer.begin(viewport);
        for layer in &self.layers {
            renderer.push_layer(layer.alpha, &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
            for (quad, paint) in layer.quads.iter().zip(layer.paints.iter()) {
                renderer.fill_quad(quad, paint);
            }
            renderer.pop_layer();
        }
        renderer.finish();
    }
}
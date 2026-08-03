use crate::{Paint, Path, Quad, Rect, Renderer, Stroke, Viewport};

pub struct Layer {
    pub quads: Vec<Quad>,
    pub paints: Vec<Paint>,
    pub paths: Vec<(Path, Paint)>,
    pub strokes: Vec<(Path, Stroke, Paint)>,
    pub images: Vec<(crate::Image, Rect)>,
    pub svgs: Vec<(crate::Svg, Rect)>,
    pub alpha: f32,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            quads: Vec::new(),
            paints: Vec::new(),
            paths: Vec::new(),
            strokes: Vec::new(),
            images: Vec::new(),
            svgs: Vec::new(),
            alpha: 1.0,
        }
    }
}

pub struct Compositor {
    layers: Vec<Layer>,
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn push_path(&mut self, path: Path, paint: Paint) {
        if let Some(layer) = self.layers.last_mut() {
            layer.paths.push((path, paint));
        }
    }

    pub fn push_stroke(&mut self, path: Path, stroke: Stroke, paint: Paint) {
        if let Some(layer) = self.layers.last_mut() {
            layer.strokes.push((path, stroke, paint));
        }
    }

    pub fn push_image(&mut self, image: crate::Image, rect: Rect) {
        if let Some(layer) = self.layers.last_mut() {
            layer.images.push((image, rect));
        }
    }

    pub fn push_svg(&mut self, svg: crate::Svg, rect: Rect) {
        if let Some(layer) = self.layers.last_mut() {
            layer.svgs.push((svg, rect));
        }
    }

    pub fn render(&mut self, renderer: &mut dyn Renderer, viewport: &Viewport) {
        renderer.begin(viewport);
        for layer in &self.layers {
            renderer.push_layer(layer.alpha, &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

            for (quad, paint) in layer.quads.iter().zip(layer.paints.iter()) {
                renderer.fill_quad(quad, paint);
            }
            for (path, paint) in &layer.paths {
                renderer.fill_path(path, paint);
            }
            for (path, stroke, paint) in &layer.strokes {
                renderer.stroke_path(path, stroke, paint);
            }
            for (image, rect) in &layer.images {
                renderer.draw_image(image, *rect);
            }
            for (svg, rect) in &layer.svgs {
                renderer.draw_svg(svg, *rect);
            }

            renderer.pop_layer();
        }
        renderer.finish();
    }
}

//! montrs-renderer: Cross-platform rendering engine for MontRS.
//!
//! Inspired by Floem's renderer trait + Dioxus's mutation boundary + iced's compositor.
//! Provides a unified `Renderer` trait with multiple backends:
//! - `wgpu` (GPU: Vulkan/Metal/DX12/WebGPU) — default
//! - `tiny-skia` (CPU software fallback)
//!
//! # Architecture
//!
//! ```text
//! Renderer trait (begin, finish, fill, stroke, draw_glyphs, clip, push_layer)
//!   ├── wgpu::WgpuRenderer    — GPU (Vulkan/Metal/DX12/WebGPU)
//!   └── tiny_skia::SkiaRenderer — CPU (software fallback)
//!
//! Compositor (layer-based):
//!   Layer stack → each layer has quads, text, meshes, images
//!   → Flattened → Renderer.draw()
//! ```

pub mod backend;
pub mod compositor;
pub mod pipeline;
pub mod viewport;

pub use compositor::*;
pub use viewport::Viewport;

/// The core rendering trait.
///
/// Implemented by each backend (wgpu, tiny-skia).
/// Inspired by Floem's Renderer trait.
pub trait Renderer: Send + Sync {
    /// Begin a new frame with the given viewport.
    fn begin(&mut self, viewport: &Viewport);

    /// Fill a quad (rectangle) with a paint style.
    fn fill_quad(&mut self, quad: &Quad, paint: &Paint);

    /// Fill a path with a paint style.
    fn fill_path(&mut self, path: &Path, paint: &Paint);

    /// Stroke a path with a stroke style and paint.
    fn stroke_path(&mut self, path: &Path, stroke: &Stroke, paint: &Paint);

    /// Draw text glyphs at a position.
    fn draw_glyphs(&mut self, pos: Point, glyphs: &[GlyphRun], paint: &Paint);

    /// Draw an image at a rectangle.
    fn draw_image(&mut self, image: &Image, rect: Rect);

    /// Draw an SVG at a rectangle.
    fn draw_svg(&mut self, svg: &Svg, rect: Rect);

    /// Set a clipping region.
    fn clip(&mut self, shape: &Shape);

    /// Clear the clipping region.
    fn clear_clip(&mut self);

    /// Push a compositing layer with alpha and transform.
    fn push_layer(&mut self, alpha: f32, transform: &[f32; 6]);

    /// Pop the current compositing layer.
    fn pop_layer(&mut self);

    /// Finish the frame and return the rendered frame data.
    fn finish(&mut self) -> Frame;
}

// ---------------------------------------------------------------------------
// Core data types
// ---------------------------------------------------------------------------

/// A rectangle defined by position and size.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// A 2D point.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// A color in RGBA with premultiplied alpha.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}

/// Paint style for filling shapes.
#[derive(Debug, Clone)]
pub struct Paint {
    pub color: Color,
    pub anti_alias: bool,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            anti_alias: true,
        }
    }
}

/// Stroke style for stroking paths.
#[derive(Debug, Clone)]
pub struct Stroke {
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            width: 1.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// A path composed of segments.
#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

impl Path {
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.segments.push(PathSegment::MoveTo(Point { x, y }));
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.segments.push(PathSegment::LineTo(Point { x, y }));
    }

    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.segments.push(PathSegment::QuadTo(Point { x: cx, y: cy }, Point { x, y }));
    }

    pub fn cubic_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        self.segments.push(PathSegment::CubicTo(
            Point { x: cx1, y: cy1 },
            Point { x: cx2, y: cy2 },
            Point { x, y },
        ));
    }

    pub fn close(&mut self) {
        self.segments.push(PathSegment::Close);
    }
}

#[derive(Debug, Clone)]
pub enum PathSegment {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CubicTo(Point, Point, Point),
    Close,
}

/// A quad (rectangle) for batched rendering.
#[derive(Debug, Clone)]
pub struct Quad {
    pub rect: Rect,
    pub corner_radius: f32,
}

/// A shape used for clipping.
#[derive(Debug, Clone)]
pub struct Shape {
    pub path: Path,
}

/// A run of text glyphs.
#[derive(Debug, Clone)]
pub struct GlyphRun {
    pub glyph_ids: Vec<u32>,
    pub positions: Vec<Point>,
    pub font_size: f32,
}

/// An image.
#[derive(Debug, Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// An SVG.
#[derive(Debug, Clone)]
pub struct Svg {
    pub data: String,
}

/// A rendered frame.
#[derive(Debug, Clone)]
pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
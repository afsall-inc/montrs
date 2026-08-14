//! Viewport management for HiDPI and resize handling.

use crate::Rect;

/// A viewport defining the visible area and scale.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub rect: Rect,
    pub scale: f32,
    pub dpi: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32, scale: f32) -> Self {
        Self {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width,
                height,
            },
            scale,
            dpi: 96.0 * scale,
        }
    }

    pub fn logical_size(&self) -> (f32, f32) {
        (self.rect.width, self.rect.height)
    }

    pub fn physical_size(&self) -> (u32, u32) {
        (
            (self.rect.width * self.scale) as u32,
            (self.rect.height * self.scale) as u32,
        )
    }
}

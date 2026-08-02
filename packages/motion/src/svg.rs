/// SVG path animation helpers.
///
/// Provides utilities for path drawing (stroke animation) and morphing,
/// inspired by how Lucide Animated Icons uses `pathLength` and `pathOffset`.

/// Calculate the total length of an SVG path string using a simple approximation.
/// On native, this can use a more accurate method; on WASM, it delegates to the browser.
pub fn approximate_path_length(_path_d: &str) -> f64 {
    // For now, return a reasonable default. In production, this would parse
    // the path commands and compute the actual length.
    100.0
}

/// Generate a CSS `stroke-dasharray` value for path drawing animation.
pub fn stroke_dasharray(length: f64) -> String {
    format!("{} {}", length, length)
}

/// Generate a CSS `stroke-dashoffset` value for a given progress (0.0 to 1.0).
pub fn stroke_dashoffset(length: f64, progress: f64) -> String {
    format!("{}", length * (1.0 - progress))
}

/// Animation variants for SVG path drawing.
/// Use with `MotionValue` or `Tween` to animate SVG path draw-in.
pub struct PathDrawAnimation {
    pub path_length: f64,
    pub progress: f64, // 0.0 to 1.0
}

impl PathDrawAnimation {
    pub fn new(path_length: f64) -> Self {
        Self {
            path_length,
            progress: 0.0,
        }
    }

    pub fn stroke_dasharray(&self) -> String {
        stroke_dasharray(self.path_length)
    }

    pub fn stroke_dashoffset(&self) -> String {
        stroke_dashoffset(self.path_length, self.progress)
    }
}
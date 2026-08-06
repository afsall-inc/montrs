/// CSS transition and animation helpers.
///
/// Provides utility functions for generating CSS transition properties,
/// will-change hints, and transform strings — all GPU-accelerated
/// through compositor-driven properties.
/// Build a CSS `transition` property value.
pub fn css_transition(
    properties: &[&str],
    duration: &str,
    easing: &str,
) -> String {
    properties
        .iter()
        .map(|p| format!("{} {} {}", p, duration, easing))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Build a CSS `will-change` property value for GPU acceleration hints.
pub fn will_change(properties: &[&str]) -> String {
    properties.join(", ")
}

/// Preset: fast opacity transition.
pub const FADE: &str = "opacity 0.15s ease-in-out";

/// Preset: smooth transform + opacity transition.
pub const SLIDE_UP: &str =
    "transform 0.3s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s ease";

/// Preset: scale transform transition.
pub const SCALE: &str = "transform 0.2s cubic-bezier(0.16, 1, 0.3, 1)";

/// Preset: spring-like transform transition.
pub const SPRING: &str = "transform 0.5s cubic-bezier(0.16, 1, 0.3, 1)";

/// Generate a CSS transform string for 3D GPU acceleration.
pub fn gpu_transform(
    translate_x: f64,
    translate_y: f64,
    scale: f64,
    rotate: f64,
) -> String {
    format!(
        "translate3d({}px, {}px, 0px) scale({}) rotate({}deg)",
        translate_x, translate_y, scale, rotate
    )
}

/// Generate a CSS transform string for 2D transforms (no GPU hint).
pub fn transform_2d(x: f64, y: f64, scale: f64, rotate: f64) -> String {
    format!(
        "translate({}px, {}px) scale({}) rotate({}deg)",
        x, y, scale, rotate
    )
}

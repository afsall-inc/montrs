#[cfg(feature = "wgpu-backend")]
pub mod wgpu;

#[cfg(feature = "tiny-skia-backend")]
pub mod tiny_skia;

/// Auto-select the best available backend.
pub fn auto_select() -> &'static str {
    #[cfg(feature = "wgpu-backend")]
    {
        "wgpu"
    }
    #[cfg(not(feature = "wgpu-backend"))]
    {
        "tiny-skia"
    }
}

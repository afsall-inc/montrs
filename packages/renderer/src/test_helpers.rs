//! Test helpers for montrs-renderer.

use crate::*;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn default_viewport() -> Viewport {
    Viewport::new(800.0, 600.0, 1.0)
}

pub fn default_paint() -> Paint {
    Paint::default()
}

pub struct TestContext;

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
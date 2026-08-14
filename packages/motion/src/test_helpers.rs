//! Test helpers for montrs-motion.

use crate::*;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn default_spring() -> Spring {
    Spring::new(100.0, 10.0, 1.0)
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
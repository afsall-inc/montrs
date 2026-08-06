//! Test helpers for montrs-orm.

use crate::*;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
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
//! Test helpers for montrs-web.

use crate::WebAdapter;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub struct TestContext {
    pub adapter: WebAdapter,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            adapter: WebAdapter::new(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
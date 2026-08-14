//! Test helpers for montrs-desktop.

use crate::DesktopAdapter;
use montrs_platform::Target;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub struct TestContext {
    pub adapter: DesktopAdapter,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            adapter: DesktopAdapter::new(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
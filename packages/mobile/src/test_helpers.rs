//! Test helpers for montrs-mobile.

use crate::MobileAdapter;
use montrs_platform::Target;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub struct TestContext {
    pub adapter: MobileAdapter,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            adapter: MobileAdapter::new(Target::MobileAndroid),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
//! Test helpers for montrs-platform.
//!
//! Provides a `TestContext` for testing platform adapter traits and types.

use montrs_platform::*;

/// Initialize tracing for tests. Safe to call multiple times.
pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("warn")
        .try_init();
}

/// A test context that provides a no-op platform adapter.
pub struct TestContext {
    pub adapter: NoopPlatformAdapter,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            adapter: NoopPlatformAdapter::new(Target::Web),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
//! Test helpers for montrs-build.

use std::path::PathBuf;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub struct TestContext {
    pub site_root: PathBuf,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            site_root: PathBuf::from("/tmp/montrs-test-site"),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
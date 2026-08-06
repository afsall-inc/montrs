//! Test helpers for montrs-build-core.

use crate::*;
use std::path::PathBuf;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub struct TestContext {
    pub project_root: PathBuf,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            project_root: PathBuf::from("/tmp/montrs-test"),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
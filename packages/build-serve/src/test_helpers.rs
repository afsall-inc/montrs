//! Test helpers for montrs-build-serve.

use crate::ServeConfig;
use std::path::PathBuf;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn default_test_config() -> ServeConfig {
    ServeConfig {
        addr: "127.0.0.1:0".to_string(),
        site_root: PathBuf::from("/tmp/montrs-test-site"),
        pkg_dir: PathBuf::from("pkg"),
    }
}

pub struct TestContext {
    pub config: ServeConfig,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            config: default_test_config(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
//! Test helpers for montrs-agentignore.

use std::path::PathBuf;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn create_temp_dir() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let path = dir.path().to_path_buf();
    (dir, path)
}

pub struct TestContext {
    pub _temp_dir: tempfile::TempDir,
    pub root: PathBuf,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        let (temp_dir, root) = create_temp_dir();
        Self {
            _temp_dir: temp_dir,
            root,
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
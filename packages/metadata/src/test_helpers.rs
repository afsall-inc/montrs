//! Test helpers for montrs-metadata.

use crate::*;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn default_metadata() -> MontrsMetadata {
    MontrsMetadata {
        project: ProjectMeta {
            name: Some("test-app".to_string()),
            version: Some("0.1.0".to_string()),
            description: Some("Test project".to_string()),
        },
        serve: ServeMeta::default(),
        build: BuildMeta::default(),
        tasks: std::collections::HashMap::new(),
    }
}

pub struct TestContext {
    pub metadata: MontrsMetadata,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            metadata: default_metadata(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
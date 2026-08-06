//! Test helpers for montrs-edge.

use crate::*;
use std::collections::HashMap;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn default_edge_request() -> EdgeRequest {
    EdgeRequest {
        method: "GET".to_string(),
        path: "/".to_string(),
        headers: HashMap::new(),
        body: None,
    }
}

pub struct TestContext {
    pub adapter: EdgeAdapter,
    pub request: EdgeRequest,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            adapter: EdgeAdapter::new(),
            request: default_edge_request(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
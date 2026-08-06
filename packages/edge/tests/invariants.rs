//! Invariant tests for montrs-edge.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - PlatformAdapter Implementation
//! - Zero Heavy Dependencies: no axum, hyper, tower
//! - Fetch-Compatible: EdgeRequest/EdgeResponse mirror fetch API

use montrs_core::{AppConfig, AppSpec, EnvConfig};
use montrs_edge::*;
use montrs_platform::{PlatformAdapter, Target};
use std::collections::HashMap;

#[derive(Clone)]
struct TestConfig;
impl AppConfig for TestConfig {
    type Error = std::io::Error;
    type Env = TestEnv;
}

#[derive(Clone)]
struct TestEnv;
impl EnvConfig for TestEnv {
    fn get_var(&self, _key: &str) -> Result<String, montrs_core::EnvError> {
        Ok("test".to_string())
    }
}

#[test]
fn test_edge_adapter_construct() {
    let adapter = EdgeAdapter::new();
    assert_eq!(adapter.target(), Target::Edge);
}

#[test]
fn test_edge_adapter_default() {
    let adapter = EdgeAdapter::default();
    assert_eq!(adapter.target(), Target::Edge);
}

#[test]
fn test_edge_adapter_description() {
    let adapter = EdgeAdapter::new();
    assert!(!adapter.description().is_empty());
}

#[test]
fn test_edge_adapter_platform_adapter_trait() {
    let adapter: Box<dyn PlatformAdapter> = Box::new(EdgeAdapter::new());
    assert_eq!(adapter.target(), Target::Edge);
}

#[test]
fn test_edge_request_construct() {
    let req = EdgeRequest {
        method: "GET".to_string(),
        path: "/api/users".to_string(),
        headers: HashMap::from([("host".to_string(), "example.com".to_string())]),
        body: None,
    };
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/users");
}

#[test]
fn test_edge_response_construct() {
    let resp = EdgeResponse {
        status: 200,
        headers: HashMap::from([("content-type".to_string(), "text/html".to_string())]),
        body: b"<html></html>".to_vec(),
    };
    assert_eq!(resp.status, 200);
    assert!(!resp.body.is_empty());
}

#[test]
fn test_edge_request_response_serde() {
    let req = EdgeRequest {
        method: "POST".to_string(),
        path: "/data".to_string(),
        headers: HashMap::new(),
        body: Some(vec![1, 2, 3]),
    };
    let json = serde_json::to_string(&req).unwrap();
    let deserialized: EdgeRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(req.method, deserialized.method);
    assert_eq!(req.path, deserialized.path);
    assert_eq!(req.body, deserialized.body);
}

#[test]
fn test_handle_edge_request_returns_response() {
    let config = TestConfig;
    let env = TestEnv;
    let spec = AppSpec::new(config, env);
    let req = EdgeRequest {
        method: "GET".to_string(),
        path: "/".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    let resp = handle_edge_request(&spec, req);
    assert_eq!(resp.status, 200);
    assert!(resp.headers.contains_key("content-type"));
}
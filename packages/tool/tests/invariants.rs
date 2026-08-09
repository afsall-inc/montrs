//! Invariant tests for montrs-tool.

use montrs_tool::*;
use std::path::Path;

#[test]
fn test_tool_request_parse() {
    let req = ToolRequest::parse("rust@1.84.0");
    assert_eq!(req.name, "rust");
    assert_eq!(req.version, Some("1.84.0".to_string()));
}

#[test]
fn test_tool_request_parse_no_version() {
    let req = ToolRequest::parse("node");
    assert_eq!(req.name, "node");
    assert_eq!(req.version, None);
}

#[test]
fn test_tool_request_parse_latest() {
    let req = ToolRequest::parse("cargo@latest");
    assert_eq!(req.name, "cargo");
    assert_eq!(req.version, Some("latest".to_string()));
}

#[test]
fn test_tool_manager_new() {
    let tm = ToolManager::new();
    assert!(tm.install_dir.to_string_lossy().contains("montrs/installs"));
    assert!(tm.shims_dir.to_string_lossy().contains("montrs/shims"));
}

#[test]
fn test_tool_manager_lookup() {
    let tm = ToolManager::new();
    let tool = tm.lookup("rust");
    assert!(tool.is_some());
    let tool = tm.lookup("nonexistent");
    assert!(tool.is_none());
}

#[test]
fn test_backend_types() {
    assert_eq!(BackendType::Core.as_str(), "core");
    assert_eq!(BackendType::GitHub.as_str(), "github");
    assert_eq!(BackendType::Cargo.as_str(), "cargo");
    assert_eq!(BackendType::Ubi.as_str(), "ubi");
}

#[test]
fn test_tool_version_construct() {
    let tv = ToolVersion {
        name: "test".to_string(),
        version: "1.0.0".to_string(),
        backend: BackendType::Core,
        url: None,
        checksum: None,
        install_path: Path::new("/tmp").to_path_buf(),
        bins: vec!["test".to_string()],
    };
    assert_eq!(tv.name, "test");
    assert_eq!(tv.version, "1.0.0");
    assert_eq!(tv.bins, vec!["test"]);
}

#[test]
fn test_create_backend() {
    let backend = create_backend("test", "core", None).unwrap();
    assert_eq!(backend.name(), "test");
    assert_eq!(backend.backend_type(), BackendType::Core);
}

#[test]
fn test_create_backend_github() {
    let backend =
        create_backend("ripgrep", "github:BurntSushi/ripgrep", None).unwrap();
    assert_eq!(backend.backend_type(), BackendType::GitHub);
}

#[test]
fn test_create_backend_default_github() {
    let backend = create_backend("unknown-tool", "unknown", None).unwrap();
    assert_eq!(backend.backend_type(), BackendType::GitHub);
}

#[test]
fn test_tool_error_display() {
    let err = ToolError::NotFound("test".to_string());
    assert!(err.to_string().contains("not found"));
    let err = ToolError::AlreadyInstalled("test".to_string());
    assert!(err.to_string().contains("Already installed"));
    let err = ToolError::NotInstalled("test".to_string());
    assert!(err.to_string().contains("Not installed"));
}

#[tokio::test]
async fn test_sha256_digest_missing_file() {
    let result = sha256_digest(Path::new("/nonexistent/file")).await;
    assert!(result.is_err());
}

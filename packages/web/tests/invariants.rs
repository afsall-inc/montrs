//! Invariant tests for montrs-web.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - PlatformAdapter Implementation
//! - WASM-First: primary target wasm32-unknown-unknown
//! - No Leptos Dependency

use montrs_platform::{PlatformAdapter, Target};
use montrs_web::*;

#[test]
fn test_web_adapter_construct() {
    let adapter = WebAdapter::new();
    assert_eq!(adapter.target(), Target::Wasm);
}

#[test]
fn test_web_adapter_default() {
    let adapter = WebAdapter::default();
    assert_eq!(adapter.target(), Target::Wasm);
}

#[test]
fn test_web_adapter_with_target() {
    let adapter = WebAdapter::with_target(Target::Wasm);
    assert_eq!(adapter.target(), Target::Wasm);
}

#[test]
#[should_panic(expected = "WebAdapter requires a web target")]
fn test_web_adapter_rejects_non_web() {
    let _adapter = WebAdapter::with_target(Target::Desktop);
}

#[test]
fn test_web_adapter_description() {
    let adapter = WebAdapter::new();
    assert!(!adapter.description().is_empty());
}

#[test]
fn test_web_adapter_noop_non_wasm() {
    let adapter = WebAdapter::new();
    adapter.open_url("https://example.com");
    adapter.set_title("test");
    adapter.set_size(1024, 768);
}

#[test]
fn test_web_adapter_platform_adapter_trait() {
    let adapter: Box<dyn PlatformAdapter> = Box::new(WebAdapter::new());
    assert_eq!(adapter.target(), Target::Wasm);
}
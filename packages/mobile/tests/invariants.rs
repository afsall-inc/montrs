//! Invariant tests for montrs-mobile.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - PlatformAdapter Implementation
//! - Feature-Gated Backends
//! - Stub-Ready: sensible no-op defaults

use montrs_mobile::*;
use montrs_platform::{PlatformAdapter, Target};

#[test]
fn test_mobile_adapter_construct() {
    let adapter = MobileAdapter::new(Target::Mobile);
    assert_eq!(adapter.target(), Target::Mobile);
}

#[test]
#[should_panic(expected = "MobileAdapter requires a mobile target")]
fn test_mobile_adapter_rejects_non_mobile() {
    let _adapter = MobileAdapter::new(Target::Web);
}

#[test]
fn test_mobile_adapter_description() {
    let adapter = MobileAdapter::new(Target::Mobile);
    assert_eq!(adapter.description(), "Mobile platform");
}

#[test]
fn test_mobile_adapter_noop_methods() {
    let adapter = MobileAdapter::new(Target::Mobile);
    adapter.open_url("https://example.com");
    adapter.set_title("test");
    adapter.set_size(800, 600);
}

#[test]
fn test_mobile_error_display() {
    let err = MobileError::Generic("test error".to_string());
    assert!(format!("{}", err).contains("Mobile error"));
}

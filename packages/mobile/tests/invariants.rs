//! Invariant tests for montrs-mobile.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - PlatformAdapter Implementation
//! - Feature-Gated Backends
//! - Stub-Ready: sensible no-op defaults

use montrs_mobile::*;
use montrs_platform::{PlatformAdapter, Target};

#[test]
fn test_mobile_adapter_construct_android() {
    let adapter = MobileAdapter::new(Target::MobileAndroid);
    assert_eq!(adapter.target(), Target::MobileAndroid);
}

#[test]
fn test_mobile_adapter_construct_ios() {
    let adapter = MobileAdapter::new(Target::MobileIos);
    assert_eq!(adapter.target(), Target::MobileIos);
}

#[test]
#[should_panic(expected = "MobileAdapter requires a mobile target")]
fn test_mobile_adapter_rejects_non_mobile() {
    let _adapter = MobileAdapter::new(Target::Server);
}

#[test]
fn test_mobile_adapter_description() {
    let android = MobileAdapter::new(Target::MobileAndroid);
    assert_eq!(android.description(), "Android mobile platform");
    let ios = MobileAdapter::new(Target::MobileIos);
    assert_eq!(ios.description(), "iOS mobile platform");
}

#[test]
fn test_mobile_adapter_noop_methods() {
    let adapter = MobileAdapter::new(Target::MobileAndroid);
    adapter.open_url("https://example.com");
    adapter.set_title("test");
    adapter.set_size(800, 600);
}

#[test]
fn test_mobile_error_display() {
    let err = MobileError::Generic("test error".to_string());
    assert!(format!("{}", err).contains("Mobile error"));
}

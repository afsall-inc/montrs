//! Invariant tests for montrs-platform.
//!
//! Validates the invariants defined in `docs/invariants.md`.

use montrs_platform::*;

#[test]
fn test_target_enum_values() {
    assert!(Target::Server.is_web());
    assert!(Target::Wasm.is_web());
    assert!(Target::Edge.is_web());
    assert!(Target::Desktop.is_desktop());
    assert!(Target::MobileAndroid.is_mobile());
    assert!(Target::MobileIos.is_mobile());
}

#[test]
fn test_target_description_not_empty() {
    for target in &[
        Target::Server,
        Target::Wasm,
        Target::Edge,
        Target::Desktop,
        Target::MobileAndroid,
        Target::MobileIos,
    ] {
        assert!(!target.description().is_empty());
    }
}

#[test]
fn test_noop_platform_adapter() {
    let adapter = NoopPlatformAdapter::new(Target::Server);
    assert_eq!(adapter.target(), Target::Server);
    assert!(!adapter.description().is_empty());
    // No-op methods should not panic
    adapter.open_url("https://example.com");
    adapter.set_title("test");
    adapter.set_size(800, 600);
}

#[test]
fn test_platform_adapter_trait_is_object_safe() {
    // Verify the trait can be used as a trait object
    let adapter: Box<dyn PlatformAdapter> =
        Box::new(NoopPlatformAdapter::new(Target::Wasm));
    assert_eq!(adapter.target(), Target::Wasm);
}
//! Invariant tests for montrs-platform.
//!
//! Validates the invariants defined in `docs/invariants.md`.

use montrs_platform::*;

#[test]
fn test_target_enum_values() {
    assert!(Target::Web.is_web());
    assert!(Target::Desktop.is_desktop());
    assert!(Target::Mobile.is_mobile());
    assert!(Target::Tui.is_tui());
}

#[test]
fn test_target_description_not_empty() {
    for target in &[Target::Web, Target::Desktop, Target::Mobile, Target::Tui] {
        assert!(!target.description().is_empty());
    }
}

#[test]
fn test_noop_platform_adapter() {
    let adapter = NoopPlatformAdapter::new(Target::Web);
    assert_eq!(adapter.target(), Target::Web);
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
        Box::new(NoopPlatformAdapter::new(Target::Web));
    assert_eq!(adapter.target(), Target::Web);
}

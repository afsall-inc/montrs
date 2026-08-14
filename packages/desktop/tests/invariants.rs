//! Invariant tests for montrs-desktop.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - PlatformAdapter Implementation
//! - Feature-Gated Backends: webview and native
//! - No Framework Logic

use montrs_desktop::*;
use montrs_platform::{PlatformAdapter, Target};

#[test]
fn test_desktop_adapter_construct() {
    let adapter = DesktopAdapter::new();
    assert_eq!(adapter.target(), Target::Desktop);
}

#[test]
fn test_desktop_adapter_default() {
    let adapter = DesktopAdapter::default();
    assert_eq!(adapter.target(), Target::Desktop);
}

#[test]
fn test_desktop_adapter_description() {
    let adapter = DesktopAdapter::new();
    assert!(!adapter.description().is_empty());
}

#[test]
fn test_desktop_adapter_platform_adapter_trait() {
    let adapter: Box<dyn PlatformAdapter> = Box::new(DesktopAdapter::new());
    assert_eq!(adapter.target(), Target::Desktop);
}

#[test]
fn test_desktop_error_display() {
    let err = DesktopError::Window("test error".to_string());
    assert!(format!("{}", err).contains("Window error"));
}

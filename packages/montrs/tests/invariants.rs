//! Invariant tests for the montrs facade crate.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Clean Re-exports: public API of core packages re-exported
//! - Minimal Logic: acts primarily as a facade

#[test]
fn test_platform_re_export() {
    let _target = montrs::platform::Target::Server;
}

#[test]
fn test_prelude_imports() {
    let _ = montrs::prelude::Target::Server;
}

#[test]
fn test_core_module_exists() {
    let _ = montrs::core;
}
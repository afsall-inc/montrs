//! Invariant tests for the montrs facade crate.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Clean Re-exports: public API of core packages re-exported
//! - Minimal Logic: acts primarily as a facade

#[test]
fn test_core_re_export() {
    // `core` is a module re-export; verify it resolves at compile time.
    fn _assert_module() {
        let _ = montrs::core::Target::Web;
    }
}

#[test]
fn test_prelude_imports() {
    let _ = montrs::prelude::Target::Web;
}

#[test]
fn test_core_module_exists() {
    // Verify the core module re-export resolves to the platform Target type.
    let _t: montrs::core::Target = montrs::core::Target::Web;
}

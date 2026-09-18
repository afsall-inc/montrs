//! MontRS hot-reload hooks (convention file).
//!
//! Wired automatically by
//! [`montrs_app_abi::export_app_with_hotpatch!`]. Register any in-memory state
//! that should survive a hot reload with `montrs_app_abi::state::register`, or
//! leave the no-op defaults. Keep your own app state in `state.rs` (or wherever
//! you like) — this file is only the bridge.

/// Called by the framework before each render.
pub fn before_render() {}

/// Serialize registered state for a reload.
pub fn export_state() -> Vec<u8> {
    montrs_app_abi::state::export()
}

/// Restore state after a reload.
pub fn import_state(bytes: &[u8]) {
    montrs_app_abi::state::import(bytes);
}

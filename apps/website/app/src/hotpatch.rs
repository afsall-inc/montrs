//! MontRS hot-reload hooks (convention file).
//!
//! The framework wires `export_state`, `import_state`, and `before_render` from
//! this module into the app's dylib entry automatically (see
//! [`montrs_app_abi::export_app_with_hotpatch!`]). Register any in-memory state
//! that should survive a hot reload with [`montrs_app_abi::state`]; the rest of
//! this file stays the same across apps. Keep your own app state in `state.rs`
//! (or wherever you like) and register it here.

use std::sync::Once;
use std::sync::atomic::{AtomicU32, Ordering};

use montrs_app_abi::state;

/// Requests served (a demo store that persists across reloads).
static HITS: AtomicU32 = AtomicU32::new(0);
static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        state::register(
            "hits",
            || HITS.load(Ordering::Relaxed).to_le_bytes().to_vec(),
            |bytes| {
                if let Ok(value) = <[u8; 4]>::try_from(bytes) {
                    HITS.store(u32::from_le_bytes(value), Ordering::Relaxed);
                }
            },
        );
    });
}

/// Called by the framework before each render.
pub fn before_render() {
    init();
    HITS.fetch_add(1, Ordering::Relaxed);
}

/// Requests served so far (for display).
pub fn hits() -> u32 {
    init();
    HITS.load(Ordering::Relaxed)
}

/// Serialize registered state for a reload (called by the framework).
pub fn export_state() -> Vec<u8> {
    init();
    state::export()
}

/// Restore state after a reload (called by the framework).
pub fn import_state(bytes: &[u8]) {
    init();
    state::import(bytes);
}

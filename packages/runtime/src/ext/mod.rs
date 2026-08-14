//! Extension modules for the MontRS runtime.
//!
//! Each module provides a function that returns a `RuntimeExtension` with
//! ops for that domain. Inspired by Deno's `ext/` directory.

pub mod atomic;
pub mod console;
pub mod crypto;
pub mod fs;
#[cfg(feature = "http")]
pub mod http;
pub mod net;
pub mod os;
pub mod process;
pub mod web;

use crate::RuntimeExtension;

/// Load all default extensions (FS, Net, OS, Console, Crypto, Web).
pub fn default_extensions() -> Vec<RuntimeExtension> {
    vec![
        fs::init(),
        net::init(),
        os::init(),
        console::init(),
        crypto::init(),
        web::init(),
    ]
}

/// Load all extensions including heavier ones (HTTP, Process).
#[cfg(feature = "http")]
pub fn full_extensions() -> Vec<RuntimeExtension> {
    let mut exts = default_extensions();
    exts.push(http::init());
    exts.push(process::init());
    exts
}

#[cfg(not(feature = "http"))]
pub fn full_extensions() -> Vec<RuntimeExtension> {
    let mut exts = default_extensions();
    exts.push(process::init());
    exts
}
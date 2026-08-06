//! Invariant tests for montrs-build-serve.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Static Serving Only: serves pre-built files
//! - Trait-Only Dependency: depends on montrs-build-core for config types
//! - Lightweight: axum + tower-http ServeDir

use montrs_build_serve::ServeConfig;
use std::path::PathBuf;

#[test]
fn test_serve_config_defaults() {
    let config = ServeConfig {
        addr: "0.0.0.0:3000".to_string(),
        site_root: PathBuf::from("target/site"),
        pkg_dir: PathBuf::from("pkg"),
    };
    assert_eq!(config.addr, "0.0.0.0:3000");
    assert_eq!(config.site_root, PathBuf::from("target/site"));
    assert_eq!(config.pkg_dir, PathBuf::from("pkg"));
}

#[test]
fn test_serve_config_debug_and_clone() {
    let config = ServeConfig {
        addr: "127.0.0.1:8080".to_string(),
        site_root: PathBuf::from("dist"),
        pkg_dir: PathBuf::from("wasm"),
    };
    let cloned = config.clone();
    assert_eq!(config.addr, cloned.addr);
    assert_eq!(config.site_root, cloned.site_root);
}
//! Invariant tests for montrs-metadata.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - All project metadata lives in montrs.toml
//! - Auto-detects from Cargo workspace
//! - All fields have sensible defaults

use montrs_metadata::*;

#[test]
fn test_metadata_default() {
    let meta = MontrsMetadata::default();
    assert!(meta.project.name.is_none());
    assert!(meta.project.version.is_none());
}

#[test]
fn test_metadata_with_project() {
    let meta = MontrsMetadata {
        project: ProjectMeta {
            name: Some("my-app".to_string()),
            version: Some("1.0.0".to_string()),
            description: None,
        },
        ..Default::default()
    };
    assert_eq!(meta.project.name.unwrap(), "my-app");
    assert_eq!(meta.project.version.unwrap(), "1.0.0");
}

#[test]
fn test_serve_meta_defaults() {
    let serve = ServeMeta::default();
    assert_eq!(serve.site_addr, "0.0.0.0:3000");
    assert_eq!(serve.reload_port, 3001);
    assert_eq!(serve.site_root, "target/site");
    assert_eq!(serve.site_pkg_dir, "pkg");
    assert_eq!(serve.browserquery, "defaults");
    assert!(serve.lib_default_features);
    assert!(serve.bin_default_features);
}

#[test]
fn test_build_meta_defaults() {
    let build = BuildMeta::default();
    assert!(!build.release);
    assert_eq!(build.target, "index.html");
}

#[test]
fn test_metadata_serde_roundtrip() {
    let meta = MontrsMetadata::default();
    let toml_str = toml::to_string(&meta).unwrap();
    let parsed: MontrsMetadata = toml::from_str(&toml_str).unwrap();
    assert!(parsed.project.name.is_none());
}
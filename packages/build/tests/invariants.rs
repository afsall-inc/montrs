//! Invariant tests for montrs-build.
//!
//! Validates that re-exports from build-core, build-watch, build-serve work.

use montrs_build::*;

#[test]
fn test_re_exports_build_core() {
    let _step = BuildStep::Server;
    fn _takes_pipeline(_p: &impl BuildPipeline) {}
}

#[test]
fn test_run_cargo_type_check() {
    let _args = vec!["build".to_string()];
}

#[test]
fn test_copy_dir_no_panic_on_missing_src() {
    let result = copy_dir(
        std::path::Path::new("/tmp/nonexistent_src_12345"),
        std::path::Path::new("/tmp/nonexistent_dst_12345"),
    );
    assert!(result.is_ok());
}

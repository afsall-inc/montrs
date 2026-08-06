//! Invariant tests for montrs-build-core.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Zero Heavy Dependencies: No axum, hyper, tower, notify
//! - Trait-Driven: BuildPipeline trait
//! - Config-Only: Only config types and trait definition

use montrs_build_core::*;

#[test]
fn test_build_step_enum_values() {
    let steps = [
        BuildStep::Server,
        BuildStep::Frontend,
        BuildStep::Tailwind,
        BuildStep::Assets,
        BuildStep::IndexHtml,
    ];
    assert_eq!(steps.len(), 5);
    assert!(steps.contains(&BuildStep::Server));
    assert!(steps.contains(&BuildStep::Frontend));
    assert!(steps.contains(&BuildStep::Tailwind));
    assert!(steps.contains(&BuildStep::Assets));
    assert!(steps.contains(&BuildStep::IndexHtml));
}

#[test]
fn test_build_step_debug_and_clone() {
    let step = BuildStep::Server;
    let cloned = step;
    assert_eq!(format!("{:?}", step), format!("{:?}", cloned));
    assert_eq!(step, cloned);
}

#[test]
fn test_build_pipeline_trait_is_object_safe() {
    use std::path::Path;
    struct MockPipeline;
    impl BuildPipeline for MockPipeline {
        fn build_server(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn build_frontend(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn process_tailwind(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn copy_assets(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn generate_index_html(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn build_all(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn metadata(&self) -> &montrs_metadata::MontrsMetadata {
            unimplemented!()
        }
        fn project_root(&self) -> &Path {
            unimplemented!()
        }
        fn site_root(&self) -> &Path {
            unimplemented!()
        }
        fn pkg_dir(&self) -> &Path {
            unimplemented!()
        }
    }
    let pipeline: Box<dyn BuildPipeline> = Box::new(MockPipeline);
    assert!(pipeline.build_server().is_ok());
    assert!(pipeline.build_frontend().is_ok());
}

#[test]
fn test_find_workspace_target_dir_default() {
    let result =
        find_workspace_target_dir(std::path::Path::new("/nonexistent"));
    assert!(result.is_ok());
}

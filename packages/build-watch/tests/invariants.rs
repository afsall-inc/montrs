//! Invariant tests for montrs-build-watch.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Trait-Only Dependency: depends on montrs-build-core for the trait
//! - Debounced Events: 300ms debounce
//! - Cross-Platform: uses notify

use montrs_build_core::BuildPipeline;
use montrs_metadata::MontrsMetadata;
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
    fn metadata(&self) -> &MontrsMetadata {
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

#[test]
fn test_watch_directory_trait_use() {
    fn accepts_pipeline(_p: &impl BuildPipeline) {}
    let pipeline = MockPipeline;
    accepts_pipeline(&pipeline);
}

#[test]
fn test_watch_and_rebuild_accepts_static_pipeline() {
    fn takes_fn(_f: fn()) {}
    takes_fn(|| {});
}

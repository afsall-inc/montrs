use montrs_build::Pipeline;
use std::path::Path;

pub async fn run() -> anyhow::Result<()> {
    let pipeline = Pipeline::from_root(Path::new("."))?;
    pipeline.build_all()
}
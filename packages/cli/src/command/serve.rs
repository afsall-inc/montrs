use montrs_build::{Pipeline, DevServer};
use std::path::Path;

pub async fn run() -> anyhow::Result<()> {
    let pipeline = Pipeline::from_root(Path::new("."))?;

    // Build everything
    pipeline.build_all()?;

    // Start dev server
    let server = DevServer::new(
        pipeline.site_root.clone(),
        &pipeline.meta.serve.site_addr,
    );
    server.run().await
}
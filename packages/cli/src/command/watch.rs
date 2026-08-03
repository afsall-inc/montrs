use montrs_build::Pipeline;
use std::path::Path;

pub async fn run() -> anyhow::Result<()> {
    let pipeline = Pipeline::from_root(Path::new("."))?;

    // Initial build
    pipeline.build_all()?;

    println!("Watching for changes...");

    // Use notify-based file watching from montrs_build
    montrs_build::watch_directory(Path::new("."), move || {
        if let Err(e) = pipeline.build_all() {
            eprintln!("Build error: {e}");
        }
    })?;

    Ok(())
}

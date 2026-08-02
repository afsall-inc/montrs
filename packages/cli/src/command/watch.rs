use montrs_build::Pipeline;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    let pipeline = Pipeline::from_root(Path::new("."))?;

    // Initial build
    pipeline.build_all()?;

    println!(" Watching for changes...");

    // Use a simple polling approach for file watching
    loop {
        std::thread::sleep(Duration::from_secs(2));
        // TODO: Implement proper file watching with notify crate
        // For now, rebuild on any keypress
        println!(" Press Ctrl+C to stop");
    }
}
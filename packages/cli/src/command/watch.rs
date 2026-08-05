use montrs_build::Pipeline;
use std::path::Path;
use tokio::process::Command as TokioCommand;

pub async fn run() -> anyhow::Result<()> {
    let pipeline = Pipeline::from_root(Path::new("."))?;

    pipeline.build_all()?;

    let addr = pipeline.meta.serve.site_addr.clone();
    let site_root = pipeline.site_root.to_string_lossy().to_string();
    let pkg_dir = pipeline.pkg_dir.to_string_lossy().to_string();

    let bin = pipeline
        .workspace_target_dir
        .join("debug")
        .join(&pipeline.server_bin_name);

    if !bin.exists() {
        anyhow::bail!(
            "SSR server binary not found at {}. Build may have failed.",
            bin.display()
        );
    }

    println!("Watching for changes and serving on {addr}...");

    let mut server_child = TokioCommand::new(&bin)
        .env("MONTRS_SITE_ROOT", &site_root)
        .env("MONTRS_SITE_PKG_DIR", &pkg_dir)
        .env("MONTRS_SITE_ADDR", &addr)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;

    montrs_build::watch_directory(Path::new("."), move || {
        println!("Change detected — rebuilding...");
        if let Err(e) = pipeline.build_all() {
            eprintln!("Build error: {e}");
        } else {
            println!("Rebuild complete.");
        }
    })?;

    server_child.wait().await?;
    Ok(())
}

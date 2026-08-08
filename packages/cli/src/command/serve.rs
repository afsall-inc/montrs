use montrs_build::{BuildPipeline, Pipeline};
use std::{path::Path, process::Command};

pub async fn run() -> anyhow::Result<()> {
    let pipeline = match Pipeline::from_root(Path::new(".")) {
        Ok(p) => p,
        Err(e) => {
            anyhow::bail!(
                "Could not find montrs.toml in the current directory. Are you \
                 in a MontRS project? Error: {e}"
            );
        }
    };

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

    println!("Starting SSR server at {addr}");
    println!("Site root: {site_root}");
    println!("PKG dir: {pkg_dir}");

    let status = Command::new(&bin)
        .env("MONTRS_SITE_ROOT", &site_root)
        .env("MONTRS_SITE_PKG_DIR", &pkg_dir)
        .env("MONTRS_SITE_ADDR", &addr)
        .env("MONTRS_RELOAD_PORT", &pipeline.meta.serve.reload_port.to_string())
        .env(
            "MONTRS_OUTPUT_NAME",
            pipeline
                .meta
                .serve
                .output_name
                .as_deref()
                .unwrap_or("website"),
        )
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("SSR server exited with error code");
    }

    Ok(())
}

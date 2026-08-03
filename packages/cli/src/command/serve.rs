use montrs_build::Pipeline;
use std::path::Path;
use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    let pipeline = match Pipeline::from_root(Path::new(".")) {
        Ok(p) => p,
        Err(e) => {
            anyhow::bail!(
                "Could not find montrs.toml in the current directory. \
                 Are you in a MontRS project? Error: {e}"
            );
        }
    };

    pipeline.build_all()?;

    let addr = pipeline.meta.serve.site_addr.clone();
    let site_root = pipeline.site_root.to_string_lossy().to_string();
    let pkg_dir = pipeline.pkg_dir.to_string_lossy().to_string();

    let bin = &pipeline.server_bin_path;

    if !bin.exists() {
        println!("No server binary found. Serving static files from {site_root}");
        let dev = montrs_build::DevServer::new(pipeline.site_root.clone(), &addr);
        dev.run().await?;
        return Ok(());
    }

    println!("Starting SSR server at {addr}");
    println!("Site root: {site_root}");
    println!("PKG dir: {pkg_dir}");

    let status = Command::new(bin)
        .env("LEPTOS_SITE_ROOT", &site_root)
        .env("LEPTOS_SITE_PKG_DIR", &pkg_dir)
        .env("LEPTOS_SITE_ADDR", &addr)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("Server exited with error code");
    }

    Ok(())
}
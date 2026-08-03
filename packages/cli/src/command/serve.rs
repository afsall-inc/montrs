use montrs_build::Pipeline;
use std::path::Path;
use std::process::Command;
use std::net::TcpListener;

/// Try to find an available port starting from the given address.
/// Increments port number until it finds a free one.
fn find_available_addr(base_addr: &str) -> String {
    // Parse the address
    let (host, port_str) = match base_addr.rsplit_once(':') {
        Some((h, p)) => (h, p),
        None => (base_addr, "3000"),
    };
    let mut port: u16 = port_str.parse().unwrap_or(3000);

    // Try up to 100 ports
    for _ in 0..100 {
        let addr = format!("{host}:{port}");
        if TcpListener::bind(&addr).is_ok() {
            return addr;
        }
        port += 1;
    }

    // Fallback
    format!("{host}:{port}")
}

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

    // Build everything
    pipeline.build_all()?;

    let addr = find_available_addr(&pipeline.meta.serve.site_addr);
    let site_root = pipeline.site_root.to_string_lossy().to_string();
    let pkg_dir = pipeline.pkg_dir.to_string_lossy().to_string();

    println!("Starting SSR server on http://{addr}");
    println!("Site root: {site_root}");
    println!("PKG dir: {pkg_dir}");

    let bin = &pipeline.server_bin_path;
    if !bin.exists() {
        // No server binary — fall back to static dev server
        println!("No server binary found. Serving static files from {site_root}");
        println!("Dev server listening on http://{addr}");
        let dev = montrs_build::DevServer::new(pipeline.site_root.clone(), &addr);
        dev.run().await?;
        return Ok(());
    }

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
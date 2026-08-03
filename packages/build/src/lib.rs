//! montrs-build: Native Rust build pipeline for MontRS applications.
//!
//! Replaces `cargo-leptos` entirely. Handles:
//! - Reading `montrs.toml` for project metadata
//! - Building the server binary with `cargo build`
//! - Building the WASM frontend with `cargo build --target wasm32-unknown-unknown`
//! - Running Tailwind CSS v4 CLI
//! - Copying assets to the site root
//! - File watching with auto-rebuild
//! - Dev server with hot-reload

pub mod pipeline;
pub mod watch;
pub mod serve;

pub use pipeline::*;
pub use watch::*;
pub use serve::*;

use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Run a cargo command and stream output.
pub fn run_cargo(args: &[String]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("cargo command failed: cargo {}", args.join(" "));
    }
    Ok(())
}

/// Run tailwindcss CLI on the input file to produce the output file.
pub fn run_tailwind(input: &Path, output: &Path) -> Result<()> {
    let status = Command::new("tailwindcss")
        .arg("-i")
        .arg(input)
        .arg("-o")
        .arg(output)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("tailwindcss failed");
    }
    Ok(())
}

/// Copy a directory recursively.
pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if src.exists() {
        fs_extra::dir::copy(
            src,
            dst,
            &fs_extra::dir::CopyOptions::new().overwrite(true).content_only(true),
        )?;
    }
    Ok(())
}
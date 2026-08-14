//! montrs-build: Facade crate for the MontRS build system.
//!
//! Re-exports `montrs-build-core`, `montrs-build-watch`, and `montrs-build-serve`
//! for convenience, and provides the concrete `Pipeline` struct that implements
//! `BuildPipeline`.

pub use montrs_build_core::*;
pub use montrs_build_serve::*;
pub use montrs_build_watch::*;

mod pipeline;

pub use pipeline::Pipeline;

/// Run a cargo command and stream output.
/// Automatically sets RUSTFLAGS to enable Leptos `erase_components`
/// for reduced type-depth and faster compiles.
pub fn run_cargo(args: &[String]) -> anyhow::Result<()> {
    let status = std::process::Command::new("cargo")
        .env("RUSTFLAGS", "--cfg erase_components")
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
pub fn run_tailwind(
    input: &std::path::Path,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let status = std::process::Command::new("tailwindcss")
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
pub fn copy_dir(
    src: &std::path::Path,
    dst: &std::path::Path,
) -> anyhow::Result<()> {
    if src.exists() {
        fs_extra::dir::copy(
            src,
            dst,
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true),
        )?;
    }
    Ok(())
}

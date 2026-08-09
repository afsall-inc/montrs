//! Shim generation — create executable stubs for MontRS-managed tools.

use std::path::{Path, PathBuf};

/// Create a shim binary wrapper for a tool.
///
/// On Unix, creates a small shell script that execs the real binary.
/// On Windows, copies the binary to the shims directory.
pub fn create_shim(
    shims_dir: &Path,
    _tool_name: &str,
    binary_name: &str,
    target_bin: &Path,
) -> std::io::Result<()> {
    std::fs::create_dir_all(shims_dir)?;
    let shim_path = shims_dir.join(binary_name);

    #[cfg(windows)]
    {
        if target_bin.exists() {
            std::fs::copy(target_bin, &shim_path)?;
        }
    }

    #[cfg(not(windows))]
    {
        let script = format!(
            "#!/bin/sh\nexec \"{}\" \"$@\"\n",
            target_bin.to_string_lossy()
        );
        std::fs::write(&shim_path, script)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(
                &shim_path,
                std::fs::Permissions::from_mode(0o755),
            )?;
        }
    }

    Ok(())
}

/// Rebuild all shims using default install and shims directories.
pub fn reshim_all_default() -> std::io::Result<usize> {
    let install_dir = montrs_tool::backend::default_install_dir();
    let shims_dir = montrs_tool::backend::default_shims_dir();
    reshim_all(&install_dir, &shims_dir)
}

/// Get the default shims directory.
pub fn default_shims_dir() -> PathBuf {
    montrs_tool::backend::default_shims_dir()
}

/// Rebuild all shims for all installed tools.
pub fn reshim_all(
    install_dir: &Path,
    shims_dir: &Path,
) -> std::io::Result<usize> {
    if !install_dir.exists() {
        return Ok(0);
    }
    let mut count = 0;
    for tool_entry in std::fs::read_dir(install_dir)? {
        let tool_entry = tool_entry?;
        if !tool_entry.path().is_dir() {
            continue;
        }
        let tool_name = tool_entry.file_name().to_string_lossy().to_string();
        // Find the latest version
        let mut versions: Vec<String> = Vec::new();
        for ver_entry in std::fs::read_dir(tool_entry.path())? {
            let ver_entry = ver_entry?;
            if ver_entry.path().is_dir() {
                versions
                    .push(ver_entry.file_name().to_string_lossy().to_string());
            }
        }
        versions.sort_by(|a, b| b.cmp(a));
        if let Some(latest) = versions.first() {
            let version_path = tool_entry.path().join(latest);
            let bin_dir = version_path.join("bin");
            if bin_dir.exists() {
                for bin_entry in std::fs::read_dir(&bin_dir)? {
                    let bin_entry = bin_entry?;
                    let bin_name =
                        bin_entry.file_name().to_string_lossy().to_string();
                    create_shim(
                        shims_dir,
                        &tool_name,
                        &bin_name,
                        &bin_entry.path(),
                    )?;
                    count += 1;
                }
            } else {
                // Tool binary might be directly in the version dir.
                create_shim(shims_dir, &tool_name, &tool_name, &version_path)?;
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Remove all shims.
pub fn remove_all_shims(shims_dir: &Path) -> std::io::Result<usize> {
    let mut count = 0;
    if !shims_dir.exists() {
        return Ok(0);
    }
    for entry in std::fs::read_dir(shims_dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            std::fs::remove_file(entry.path())?;
            count += 1;
        }
    }
    Ok(count)
}

/// Get the list of all shim binaries.
pub fn list_shims(shims_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut shims = Vec::new();
    if !shims_dir.exists() {
        return Ok(shims);
    }
    for entry in std::fs::read_dir(shims_dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            shims.push(entry.path());
        }
    }
    Ok(shims)
}

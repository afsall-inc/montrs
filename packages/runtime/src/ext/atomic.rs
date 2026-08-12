//! Atomic write helper for runtime FS ops — write file atomically via temp + rename.

use std::path::Path;

/// Write a file atomically: write to a temp file in the same directory,
/// then rename over the target. Prevents torn writes.
pub fn write_atomic(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("tmp");
    let tmp = dir.join(format!(".{file_name}.tmp{}", std::process::id()));

    std::fs::write(&tmp, contents)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_atomic() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target.txt");
        write_atomic(&target, b"hello").unwrap();
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "hello");
        // No temp file left behind.
        let leftover = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".tmp"))
            .count();
        assert_eq!(leftover, 0);
    }
}
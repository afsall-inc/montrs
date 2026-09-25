// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! On-disk cache store.
//!
//! A cache entry records the *fingerprint* of a step's outputs: for each output
//! its relative path, byte length, and content hash. A step is fresh when every
//! recorded output still exists with the same hash. Outputs are not copied, so
//! the cache stays cheap even for multi-megabyte WASM bundles.

use crate::{error::CacheError, key::CacheKey};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// One recorded output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputFingerprint {
    /// Path relative to the cache's base directory (`/` separators).
    pub path: String,
    /// Byte length.
    pub len: u64,
    /// Lowercase hex SHA-256 of the content.
    pub hash: String,
}

/// The manifest written for a cached key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheManifest {
    /// Namespace of the key.
    pub namespace: String,
    /// Hex digest of the key.
    pub key: String,
    /// Creation time (Unix seconds; informational only).
    pub created_unix: u64,
    /// Fingerprints of every output.
    pub outputs: Vec<OutputFingerprint>,
}

/// Aggregate cache statistics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CacheStats {
    /// Number of cached entries.
    pub entries: usize,
    /// Total bytes described by manifests (output sizes).
    pub described_bytes: u64,
}

/// A local, project-scoped content-addressed cache.
pub struct Cache {
    root: PathBuf,
    base: PathBuf,
    enabled: bool,
}

impl Cache {
    /// A cache rooted at `<project_root>/.montrs/cache`, with outputs resolved
    /// relative to `<project_root>`.
    pub fn local(project_root: &Path) -> Self {
        Self {
            root: project_root.join(".montrs").join("cache"),
            base: project_root.to_path_buf(),
            enabled: true,
        }
    }

    /// A cache with an explicit root and output base directory.
    pub fn with_root(
        root: impl Into<PathBuf>,
        base: impl Into<PathBuf>,
    ) -> Self {
        Self {
            root: root.into(),
            base: base.into(),
            enabled: true,
        }
    }

    /// A cache that never stores or reports hits. Useful in tests and when
    /// caching is disabled by configuration.
    pub fn disabled() -> Self {
        Self {
            root: PathBuf::new(),
            base: PathBuf::new(),
            enabled: false,
        }
    }

    /// Whether this cache is active.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// The cache root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn entry_dir(&self, key: &CacheKey) -> PathBuf {
        self.root.join(key.relative_path())
    }

    fn manifest_path(&self, key: &CacheKey) -> PathBuf {
        self.entry_dir(key).join("manifest.json")
    }

    /// Read the manifest for a key, if present and parseable.
    pub fn manifest(&self, key: &CacheKey) -> Option<CacheManifest> {
        if !self.enabled {
            return None;
        }
        let bytes = std::fs::read(self.manifest_path(key)).ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    /// Whether every recorded output still matches. A missing manifest is a
    /// miss; a manifest with zero outputs is a hit (the step is a no-op).
    pub fn is_fresh(&self, key: &CacheKey) -> bool {
        let Some(manifest) = self.manifest(key) else {
            return false;
        };
        manifest.outputs.iter().all(|out| {
            let path = self.base.join(&out.path);
            match fingerprint(&path) {
                Some(fp) => fp.len == out.len && fp.hash == out.hash,
                None => false,
            }
        })
    }

    /// Record the current fingerprint of `outputs` for `key`.
    pub fn record(
        &self,
        key: &CacheKey,
        outputs: &[PathBuf],
    ) -> Result<(), CacheError> {
        if !self.enabled {
            return Ok(());
        }
        let mut fps = Vec::with_capacity(outputs.len());
        for out in outputs {
            if let Some(fp) = self.fingerprint_relative(out) {
                fps.push(fp);
            }
        }
        let manifest = CacheManifest {
            namespace: key.namespace().to_string(),
            key: key.digest_hex(),
            created_unix: now_unix(),
            outputs: fps,
        };
        self.write_manifest(key, &manifest)
    }

    fn fingerprint_relative(&self, path: &Path) -> Option<OutputFingerprint> {
        let fp = fingerprint(path)?;
        let rel = path
            .strip_prefix(&self.base)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        Some(OutputFingerprint { path: rel, ..fp })
    }

    fn write_manifest(
        &self,
        key: &CacheKey,
        manifest: &CacheManifest,
    ) -> Result<(), CacheError> {
        let dir = self.entry_dir(key);
        std::fs::create_dir_all(&dir)?;
        // Best-effort exclusive lock so concurrent CLI runs do not interleave.
        let lock_path = dir.join(".lock");
        let lock = File::create(&lock_path)?;
        let _ = lock.lock_exclusive();

        let json = serde_json::to_vec_pretty(manifest)?;
        let tmp = dir.join("manifest.json.tmp");
        {
            let mut file = File::create(&tmp)?;
            file.write_all(&json)?;
            file.sync_all()?;
        }
        std::fs::rename(&tmp, self.manifest_path(key))?;
        let _ = lock.unlock();
        Ok(())
    }

    /// Invalidate one key by removing its entry.
    pub fn invalidate(&self, key: &CacheKey) -> Result<(), CacheError> {
        if !self.enabled {
            return Ok(());
        }
        let dir = self.entry_dir(key);
        if dir.exists() {
            std::fs::remove_dir_all(&dir)?;
        }
        Ok(())
    }

    /// Remove every entry in a namespace.
    pub fn clear_namespace(&self, namespace: &str) -> Result<(), CacheError> {
        if !self.enabled {
            return Ok(());
        }
        let dir = self.root.join(namespace);
        if dir.exists() {
            std::fs::remove_dir_all(&dir)?;
        }
        Ok(())
    }

    /// Remove the whole cache.
    pub fn clear_all(&self) -> Result<(), CacheError> {
        if !self.enabled {
            return Ok(());
        }
        if self.root.exists() {
            std::fs::remove_dir_all(&self.root)?;
        }
        Ok(())
    }

    /// Count entries and described output bytes.
    pub fn stats(&self) -> CacheStats {
        let mut stats = CacheStats::default();
        if !self.enabled || !self.root.exists() {
            return stats;
        }
        for entry in walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_name() == "manifest.json")
        {
            let Ok(bytes) = std::fs::read(entry.path()) else {
                continue;
            };
            let Ok(manifest) = serde_json::from_slice::<CacheManifest>(&bytes)
            else {
                continue;
            };
            stats.entries += 1;
            stats.described_bytes +=
                manifest.outputs.iter().map(|o| o.len).sum::<u64>();
        }
        stats
    }
}

/// Compute the fingerprint of a file, or `None` if unreadable.
fn fingerprint(path: &Path) -> Option<OutputFingerprint> {
    let bytes = std::fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Some(OutputFingerprint {
        path: String::new(),
        len: bytes.len() as u64,
        hash: hex::encode(hasher.finalize()),
    })
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Input;
    use std::fs;

    fn key_for(inputs: &[Input]) -> CacheKey {
        CacheKey::new("test", inputs).unwrap()
    }

    #[test]
    fn miss_then_hit_then_change() {
        let project = tempfile::tempdir().unwrap();
        let cache = Cache::local(project.path());

        let input = project.path().join("src.rs");
        fs::write(&input, "fn main() {}").unwrap();
        let out = project.path().join("out.bin");
        fs::write(&out, "artifact").unwrap();

        let key = key_for(&[Input::file(&input)]);
        assert!(!cache.is_fresh(&key), "no manifest yet -> miss");

        cache.record(&key, std::slice::from_ref(&out)).unwrap();
        assert!(cache.is_fresh(&key), "recorded output -> hit");

        // Output content changed outside the cache -> miss.
        fs::write(&out, "changed").unwrap();
        assert!(!cache.is_fresh(&key));

        // Restore content -> hit again.
        fs::write(&out, "artifact").unwrap();
        assert!(cache.is_fresh(&key));

        // Different inputs -> different key -> miss.
        std::fs::write(&input, "fn main() { println!(); }").unwrap();
        let other = key_for(&[Input::file(&input)]);
        assert!(!cache.is_fresh(&other));
    }

    #[test]
    fn missing_output_is_a_miss() {
        let project = tempfile::tempdir().unwrap();
        let cache = Cache::local(project.path());
        let out = project.path().join("gone.bin");
        fs::write(&out, "x").unwrap();
        let key = key_for(&[]);
        cache.record(&key, std::slice::from_ref(&out)).unwrap();
        assert!(cache.is_fresh(&key));
        fs::remove_file(&out).unwrap();
        assert!(!cache.is_fresh(&key));
    }

    #[test]
    fn empty_outputs_are_a_hit() {
        let project = tempfile::tempdir().unwrap();
        let cache = Cache::local(project.path());
        let key = key_for(&[Input::value("noop")]);
        cache.record(&key, &[]).unwrap();
        assert!(cache.is_fresh(&key));
    }

    #[test]
    fn invalidate_and_clear() {
        let project = tempfile::tempdir().unwrap();
        let cache = Cache::local(project.path());
        let out = project.path().join("o");
        fs::write(&out, "1").unwrap();
        let key = key_for(&[]);
        cache.record(&key, &[out]).unwrap();

        let stats = cache.stats();
        assert_eq!(stats.entries, 1);

        cache.invalidate(&key).unwrap();
        assert!(!cache.is_fresh(&key));

        cache.record(&key, &[project.path().join("o")]).unwrap();
        cache.clear_namespace("test").unwrap();
        assert!(!cache.is_fresh(&key));
    }

    #[test]
    fn disabled_cache_never_hits() {
        let cache = Cache::disabled();
        let key = key_for(&[]);
        assert!(!cache.is_fresh(&key));
        cache.record(&key, &[]).unwrap();
        assert!(!cache.is_fresh(&key));
    }
}

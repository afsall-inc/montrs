//! Code cache — caches compiled module representations to avoid re-parsing.
//!
//! Inspired by Deno's `runtime/code_cache.rs`. Modules are keyed by their
//! source hash; a cache hit skips re-compilation of the bytecode.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// A cached module entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CachedModule {
    /// The compiled code bytes (e.g. serialized bytecode).
    pub code: Vec<u8>,
    /// SHA-256 hex hash of the source.
    pub source_hash: String,
    /// Monotonic insertion sequence (FIFO eviction).
    #[serde(default)]
    pub seq: u64,
    /// When this entry was cached (unix seconds).
    pub cached_at: u64,
}

/// An LRU-ish code cache with configurable capacity and optional persistence.
pub struct CodeCache {
    entries: HashMap<String, CachedModule>,
    max_entries: usize,
    next_seq: u64,
    /// Optional file backing store (JSON).
    backing: Option<PathBuf>,
}

impl CodeCache {
    /// Create a new in-memory code cache.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
            next_seq: 1,
            backing: None,
        }
    }

    /// Create a code cache backed by a JSON file.
    pub fn new_persistent(max_entries: usize, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let mut cache = Self {
            entries: HashMap::new(),
            max_entries,
            next_seq: 1,
            backing: Some(path.clone()),
        };
        cache.load_from_disk(&path);
        // Resume sequence past any loaded entries.
        if let Some(max) = cache.entries.values().map(|e| e.seq).max() {
            cache.next_seq = max + 1;
        }
        cache
    }

    /// Hash source code to a cache key.
    pub fn hash_source(source: &str) -> String {
        use sha2::{Digest, Sha256};
        hex::encode(Sha256::digest(source.as_bytes()))
    }

    /// Get a cached module if the source hash matches.
    pub fn get(&self, key: &str, source_hash: &str) -> Option<&CachedModule> {
        self.entries.get(key).and_then(|m| {
            if m.source_hash == source_hash {
                Some(m)
            } else {
                None
            }
        })
    }

    /// Insert a new cached module. Evicts the oldest entry if at capacity.
    pub fn insert(&mut self, key: String, source: &str, code: Vec<u8>) {
        let entry = CachedModule {
            code,
            source_hash: Self::hash_source(source),
            seq: self.next_seq,
            cached_at: now_unix(),
        };
        // Re-inserting updates seq, treating it as newest.
        self.next_seq += 1;
        if self.entries.len() >= self.max_entries && !self.entries.contains_key(&key) {
            // Evict the oldest by sequence (FIFO — deterministic).
            if let Some(oldest) = self
                .entries
                .iter()
                .min_by_key(|(_, m)| m.seq)
                .map(|(k, _)| k.clone())
            {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(key, entry);
        self.persist();
    }

    /// Check if a key exists (without verifying hash).
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear the cache (and remove the backing file if present).
    pub fn clear(&mut self) {
        self.entries.clear();
        if let Some(path) = &self.backing {
            let _ = std::fs::remove_file(path);
        }
    }

    // ── Persistence ────────────────────────────────────────────────────

    pub fn persist(&self) {
        if let Some(path) = &self.backing {
            if let Ok(json) = serde_json::to_string(&self.entries) {
                let tmp = path.with_extension("tmp");
                if std::fs::write(&tmp, json).is_ok() {
                    let _ = std::fs::rename(&tmp, path);
                }
            }
        }
    }

    fn load_from_disk(&mut self, path: &Path) {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(entries) = serde_json::from_str::<HashMap<String, CachedModule>>(&content) {
                self.entries = entries;
            }
        }
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl Default for CodeCache {
    fn default() -> Self {
        Self::new(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_roundtrip() {
        let h1 = CodeCache::hash_source("fn main() {}");
        let h2 = CodeCache::hash_source("fn main() {}");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn test_insert_get() {
        let mut cache = CodeCache::new(10);
        cache.insert("mod.js".into(), "code_v1", vec![1, 2, 3]);
        let hit = cache.get("mod.js", &CodeCache::hash_source("code_v1"));
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().code, vec![1, 2, 3]);

        // Stale hash misses.
        let miss = cache.get("mod.js", &CodeCache::hash_source("code_v2"));
        assert!(miss.is_none());
    }

    #[test]
    fn test_eviction() {
        let mut cache = CodeCache::new(2);
        cache.insert("a".into(), "a", vec![1]);
        cache.insert("b".into(), "b", vec![2]);
        cache.insert("c".into(), "c", vec![3]);
        // "a" was evicted (oldest).
        assert!(!cache.contains("a"));
        assert!(cache.contains("b"));
        assert!(cache.contains("c"));
    }

    #[test]
    fn test_persistent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cache.json");
        {
            let mut cache = CodeCache::new_persistent(10, &path);
            cache.insert("mod.js".into(), "src", vec![9, 9]);
        }
        let cache2 = CodeCache::new_persistent(10, &path);
        let hit = cache2.get("mod.js", &CodeCache::hash_source("src"));
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().code, vec![9, 9]);
    }
}
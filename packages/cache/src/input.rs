// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Cache inputs and their deterministic hashing.
//!
//! An [`Input`] is one reason a build step's output could change: a file, a
//! directory tree, a glob, an inline value, an environment variable, or a tool
//! version. Inputs are hashed in a fixed order with length-prefixed records so
//! the resulting key is stable across machines and checkouts.

use crate::error::CacheError;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A single reason a cached step could be invalidated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    /// A single file (hashed by content).
    File(PathBuf),
    /// A directory tree (every file under it, sorted, hashed by content).
    Dir(PathBuf),
    /// A glob relative to `root`.
    Glob {
        /// Directory the pattern is evaluated from.
        root: PathBuf,
        /// Glob pattern (e.g. `**/*.rs`).
        pattern: String,
    },
    /// An inline value (e.g. a command line or a constant).
    Value(String),
    /// An environment variable read at hash time.
    Env(String),
    /// A named tool version.
    Tool {
        /// Tool name.
        name: String,
        /// Resolved version string.
        version: String,
    },
}

impl Input {
    /// A file input.
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File(path.into())
    }

    /// A directory input.
    pub fn dir(path: impl Into<PathBuf>) -> Self {
        Self::Dir(path.into())
    }

    /// A glob input.
    pub fn glob(root: impl Into<PathBuf>, pattern: impl Into<String>) -> Self {
        Self::Glob {
            root: root.into(),
            pattern: pattern.into(),
        }
    }

    /// An environment-variable input.
    pub fn env(name: impl Into<String>) -> Self {
        Self::Env(name.into())
    }

    /// A tool-version input.
    pub fn tool(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self::Tool {
            name: name.into(),
            version: version.into(),
        }
    }

    /// An inline-value input.
    pub fn value(value: impl Into<String>) -> Self {
        Self::Value(value.into())
    }

    fn hash_into(&self, hasher: &mut Sha256) -> Result<(), CacheError> {
        match self {
            Input::File(path) => {
                tag(hasher, "file");
                let display = normalize(path);
                field(hasher, display.as_bytes());
                match std::fs::read(path) {
                    Ok(bytes) => field(hasher, &bytes),
                    Err(_) => tag(hasher, "missing"),
                }
            }
            Input::Dir(path) => {
                tag(hasher, "dir");
                let display = normalize(path);
                field(hasher, display.as_bytes());
                for entry in sorted_files(path) {
                    let rel = entry
                        .strip_prefix(path)
                        .unwrap_or(&entry)
                        .to_string_lossy()
                        .replace('\\', "/");
                    field(hasher, rel.as_bytes());
                    match std::fs::read(&entry) {
                        Ok(bytes) => field(hasher, &bytes),
                        Err(_) => tag(hasher, "unreadable"),
                    }
                }
            }
            Input::Glob { root, pattern } => {
                tag(hasher, "glob");
                field(hasher, normalize(root).as_bytes());
                field(hasher, pattern.as_bytes());
                let joined = root.join(pattern);
                let joined = joined.to_string_lossy().replace('\\', "/");
                let mut matches: Vec<PathBuf> = glob::glob(&joined)
                    .map_err(|e| CacheError::InvalidGlob {
                        pattern: pattern.clone(),
                        reason: e.to_string(),
                    })?
                    .filter_map(Result::ok)
                    .filter(|p| p.is_file())
                    .collect();
                matches.sort();
                for path in matches {
                    field(
                        hasher,
                        path.to_string_lossy().replace('\\', "/").as_bytes(),
                    );
                    match std::fs::read(&path) {
                        Ok(bytes) => field(hasher, &bytes),
                        Err(_) => tag(hasher, "unreadable"),
                    }
                }
            }
            Input::Value(value) => {
                tag(hasher, "value");
                field(hasher, value.as_bytes());
            }
            Input::Env(name) => {
                tag(hasher, "env");
                field(hasher, name.as_bytes());
                match std::env::var(name) {
                    Ok(value) => {
                        tag(hasher, "set");
                        field(hasher, value.as_bytes());
                    }
                    Err(_) => tag(hasher, "unset"),
                }
            }
            Input::Tool { name, version } => {
                tag(hasher, "tool");
                field(hasher, name.as_bytes());
                field(hasher, version.as_bytes());
            }
        }
        Ok(())
    }
}

/// Hash a set of inputs into a stable 32-byte digest.
pub fn hash_inputs(inputs: &[Input]) -> Result<[u8; 32], CacheError> {
    let mut hasher = Sha256::new();
    // Length of the input list first, so trivially different sets differ.
    field(&mut hasher, &(inputs.len() as u64).to_le_bytes());
    for input in inputs {
        input.hash_into(&mut hasher)?;
    }
    Ok(hasher.finalize().into())
}

/// Write a tagged record: `tag` length + bytes.
fn tag(hasher: &mut Sha256, tag: &str) {
    field(hasher, tag.as_bytes());
}

/// Write a length-prefixed field so concatenation is unambiguous.
fn field(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

/// Every file under `root`, sorted by absolute path, deterministically.
fn sorted_files(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();
    files.sort();
    files
}

/// A stable, portable rendering of a path (`/` separators).
fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn identical_inputs_hash_equally() {
        let a = vec![Input::value("hello"), Input::env("PATH")];
        let b = vec![Input::value("hello"), Input::env("PATH")];
        assert_eq!(hash_inputs(&a).unwrap(), hash_inputs(&b).unwrap());
    }

    #[test]
    fn order_and_length_matter() {
        let a = vec![Input::value("ab"), Input::value("c")];
        let b = vec![Input::value("a"), Input::value("bc")];
        assert_ne!(hash_inputs(&a).unwrap(), hash_inputs(&b).unwrap());
    }

    #[test]
    fn file_content_changes_hash() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "one").unwrap();
        let first = hash_inputs(&[Input::file(&file)]).unwrap();
        fs::write(&file, "two").unwrap();
        let second = hash_inputs(&[Input::file(&file)]).unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn missing_file_is_distinct_from_empty() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nope");
        let empty = dir.path().join("empty");
        fs::write(&empty, "").unwrap();
        assert_ne!(
            hash_inputs(&[Input::file(&missing)]).unwrap(),
            hash_inputs(&[Input::file(&empty)]).unwrap()
        );
    }

    #[test]
    fn dir_hash_includes_nested_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("nested")).unwrap();
        fs::write(dir.path().join("nested/x.rs"), "fn a() {}").unwrap();
        let first = hash_inputs(&[Input::dir(dir.path())]).unwrap();
        fs::write(dir.path().join("nested/x.rs"), "fn b() {}").unwrap();
        let second = hash_inputs(&[Input::dir(dir.path())]).unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn glob_input_matches_are_hashed() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.rs"), "1").unwrap();
        fs::write(dir.path().join("b.rs"), "2").unwrap();
        fs::write(dir.path().join("c.txt"), "3").unwrap();
        let glob = vec![Input::glob(dir.path(), "**/*.rs")];
        let first = hash_inputs(&glob).unwrap();
        fs::write(dir.path().join("b.rs"), "changed").unwrap();
        assert_ne!(first, hash_inputs(&glob).unwrap());
        // Non-matching file changes do not affect the hash.
        let before = hash_inputs(&glob).unwrap();
        fs::write(dir.path().join("c.txt"), "changed").unwrap();
        assert_eq!(before, hash_inputs(&glob).unwrap());
    }

    #[test]
    fn tool_and_value_inputs_differ() {
        let a = vec![Input::tool("wasm-bindgen", "0.2.1")];
        let b = vec![Input::tool("wasm-bindgen", "0.2.2")];
        assert_ne!(hash_inputs(&a).unwrap(), hash_inputs(&b).unwrap());
    }

    #[test]
    fn invalid_glob_reports_error() {
        let bad = vec![Input::glob(".", "[")];
        assert!(matches!(
            hash_inputs(&bad),
            Err(CacheError::InvalidGlob { .. })
        ));
    }
}

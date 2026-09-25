// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Cache keys: a namespace plus the content hash of its inputs.

use crate::{
    error::CacheError,
    input::{Input, hash_inputs},
};
use std::path::Path;

/// A stable identifier for one cached unit of work.
///
/// The key is `namespace` plus a 32-byte digest of every [`Input`]. Two builds
/// with identical namespace, inputs, environment, and tool versions produce
/// the same key on any machine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    namespace: String,
    digest: [u8; 32],
}

impl CacheKey {
    /// Build a key from a namespace and inputs.
    pub fn new(
        namespace: impl Into<String>,
        inputs: &[Input],
    ) -> Result<Self, CacheError> {
        Ok(Self {
            namespace: namespace.into(),
            digest: hash_inputs(inputs)?,
        })
    }

    /// Start a builder for a namespace.
    pub fn builder(namespace: impl Into<String>) -> CacheKeyBuilder {
        CacheKeyBuilder {
            namespace: namespace.into(),
            inputs: Vec::new(),
        }
    }

    /// The namespace this key belongs to.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The lowercase hex digest of the inputs.
    pub fn digest_hex(&self) -> String {
        hex::encode(self.digest)
    }

    /// The relative path (under a cache root) for this key.
    pub fn relative_path(&self) -> std::path::PathBuf {
        Path::new(&self.namespace).join(self.digest_hex())
    }
}

/// Fluent builder for [`CacheKey`].
#[derive(Debug, Clone)]
pub struct CacheKeyBuilder {
    namespace: String,
    inputs: Vec<Input>,
}

impl CacheKeyBuilder {
    /// Add a single input.
    pub fn input(mut self, input: Input) -> Self {
        self.inputs.push(input);
        self
    }

    /// Add several inputs.
    pub fn inputs(mut self, inputs: impl IntoIterator<Item = Input>) -> Self {
        self.inputs.extend(inputs);
        self
    }

    /// Add file inputs.
    pub fn files(
        self,
        paths: impl IntoIterator<Item = impl Into<std::path::PathBuf>>,
    ) -> Self {
        self.inputs(paths.into_iter().map(Input::file))
    }

    /// Add directory inputs.
    pub fn dirs(
        self,
        paths: impl IntoIterator<Item = impl Into<std::path::PathBuf>>,
    ) -> Self {
        self.inputs(paths.into_iter().map(Input::dir))
    }

    /// Add environment-variable inputs.
    pub fn env(
        self,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.inputs(names.into_iter().map(Input::env))
    }

    /// Add a tool-version input.
    pub fn tool(
        self,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        self.input(Input::tool(name, version))
    }

    /// Add an inline-value input.
    pub fn value(self, value: impl Into<String>) -> Self {
        self.input(Input::value(value))
    }

    /// Add a glob input.
    pub fn glob(
        self,
        root: impl Into<std::path::PathBuf>,
        pattern: impl Into<String>,
    ) -> Self {
        self.input(Input::glob(root, pattern))
    }

    /// Finish and hash.
    pub fn build(self) -> Result<CacheKey, CacheError> {
        CacheKey::new(self.namespace, &self.inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn same_inputs_same_key() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a");
        fs::write(&file, "content").unwrap();

        let a = CacheKey::builder("wasm")
            .files([&file])
            .env(["PATH"])
            .tool("rustc", "1.95.0")
            .build()
            .unwrap();
        let b = CacheKey::builder("wasm")
            .files([&file])
            .env(["PATH"])
            .tool("rustc", "1.95.0")
            .build()
            .unwrap();
        assert_eq!(a, b);
        assert_eq!(a.digest_hex().len(), 64);
        assert!(a.relative_path().starts_with("wasm"));
    }

    #[test]
    fn namespace_changes_key() {
        let a = CacheKey::new("wasm", &[]).unwrap();
        let b = CacheKey::new("bindgen", &[]).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn version_bump_invalidates() {
        let a = CacheKey::builder("t").tool("tw", "4.0").build().unwrap();
        let b = CacheKey::builder("t").tool("tw", "4.1").build().unwrap();
        assert_ne!(a, b);
    }
}

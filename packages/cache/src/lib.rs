// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # montrs-cache
//!
//! A small, deterministic, content-addressed cache for MontRS build steps and
//! task runs.
//!
//! A unit of work is identified by a [`CacheKey`]: a namespace plus the content
//! hash of its [`Input`]s (files, directories, globs, environment variables,
//! tool versions, inline values). When the key is unchanged, the step's outputs
//! still match their recorded fingerprints and the step can be skipped.
//!
//! The design follows one rule above all: **when in doubt, miss.** Caching must
//! never serve a stale artifact.
//!
//! ```rust
//! use montrs_cache::{Cache, CacheKey, Input};
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let cache = Cache::local(Path::new("."));
//! let key = CacheKey::builder("example")
//!     .input(Input::value("some input"))
//!     .tool("rustc", "1.95.0")
//!     .build()?;
//! if cache.is_fresh(&key) {
//!     // skip the work
//! } else {
//!     // run the work, then:
//!     // cache.record(&key, &[output])?;
//! }
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod input;
pub mod key;
pub mod store;

pub use error::CacheError;
pub use input::{Input, hash_inputs};
pub use key::{CacheKey, CacheKeyBuilder};
pub use store::{Cache, CacheManifest, CacheStats, OutputFingerprint};

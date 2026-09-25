// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Errors for the MontRS incremental cache.

use montrs_core::AgentError;
use thiserror::Error;

/// Errors from cache operations.
#[derive(Debug, Error)]
pub enum CacheError {
    /// I/O failure while reading inputs or writing the cache.
    #[error("cache I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization failure of a manifest.
    #[error("cache serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// A glob pattern could not be parsed.
    #[error("invalid glob pattern {pattern:?}: {reason}")]
    InvalidGlob {
        /// The offending pattern.
        pattern: String,
        /// Why it is invalid.
        reason: String,
    },
}

impl AgentError for CacheError {
    fn error_code(&self) -> &'static str {
        match self {
            CacheError::Io(_) => "CACHE_IO",
            CacheError::Serialization(_) => "CACHE_SERIALIZATION",
            CacheError::InvalidGlob { .. } => "CACHE_INVALID_GLOB",
        }
    }

    fn explanation(&self) -> String {
        match self {
            CacheError::Io(e) => {
                format!("An I/O error occurred during caching: {e}.")
            }
            CacheError::Serialization(e) => {
                format!("Failed to (de)serialize a cache manifest: {e}.")
            }
            CacheError::InvalidGlob { pattern, reason } => format!(
                "The cache input glob {pattern:?} is invalid: {reason}."
            ),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            CacheError::Io(_) => vec![
                "Verify the `.montrs/cache` directory is writable.".to_string(),
                "Ensure no other process holds a lock on the cache."
                    .to_string(),
            ],
            CacheError::Serialization(_) => vec![
                "Delete `.montrs/cache` to regenerate manifests.".to_string(),
            ],
            CacheError::InvalidGlob { .. } => vec![
                "Fix the glob in the task/step input declaration.".to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "cache"
    }
}

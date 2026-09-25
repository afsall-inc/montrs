// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Local task run cache.
//!
//! A thin task-aware wrapper over [`montrs_cache::Cache`]: it derives a key from
//! a [`Task`] and, when the task is cacheable, skips it if its inputs and
//! declared outputs are unchanged.

use crate::{
    hash::{cache_outputs, resolve_outputs, task_key, task_root},
    types::Task,
};
use montrs_cache::Cache;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// A per-project cache of task results.
pub struct TaskRunCache {
    cache: Cache,
}

impl TaskRunCache {
    /// A cache rooted at `<project_root>/.montrs/cache`.
    ///
    /// Returns `None` when caching is disabled by `MONTRS_NO_CACHE` or the task
    /// has no project root.
    pub fn for_project(project_root: &Path) -> Self {
        if std::env::var_os("MONTRS_NO_CACHE").is_some() {
            return Self {
                cache: Cache::disabled(),
            };
        }
        Self {
            cache: Cache::local(project_root),
        }
    }

    /// A cache that never hits.
    pub fn disabled() -> Self {
        Self {
            cache: Cache::disabled(),
        }
    }

    /// Whether the underlying cache is active.
    pub fn is_enabled(&self) -> bool {
        self.cache.is_enabled()
    }

    /// Whether `task` is fresh and can be skipped.
    ///
    /// Returns `None` when the task is not cacheable (caching disabled, no
    /// declared outputs, or no project root).
    pub fn is_fresh(
        &self,
        task: &Task,
        dep_hashes: &BTreeMap<String, String>,
    ) -> Option<bool> {
        if !self.cache.is_enabled() {
            return None;
        }
        cache_outputs(task)?;
        let key = task_key(task, dep_hashes)?;
        Some(self.cache.is_fresh(&key))
    }

    /// Record the task's declared outputs after a successful run.
    pub fn record(
        &self,
        task: &Task,
        dep_hashes: &BTreeMap<String, String>,
    ) -> anyhow::Result<()> {
        if !self.cache.is_enabled() {
            return Ok(());
        }
        let Some(globs) = cache_outputs(task) else {
            return Ok(());
        };
        let Some(key) = task_key(task, dep_hashes) else {
            return Ok(());
        };
        let root = task_root(task);
        let outputs = resolve_outputs(&root, &globs);
        // A declared output that does not exist means the task did not produce
        // what it promised; do not record (so it will run again next time).
        if outputs.len() < globs.len() {
            return Ok(());
        }
        self.cache
            .record(&key, &outputs)
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// The base directory for recorded outputs.
    pub fn output_base(task: &Task) -> PathBuf {
        task_root(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RunEntry, TaskCacheConfig, TaskOutputs};
    use std::fs;

    fn task_in(dir: &Path, name: &str) -> Task {
        Task {
            name: name.to_string(),
            command: vec![RunEntry::Script("echo hi".to_string())],
            sources: vec!["**/*.rs".to_string()],
            outputs: TaskOutputs::Files(vec!["out.txt".to_string()]),
            cache: Some(TaskCacheConfig {
                enabled: true,
                audit: false,
                env: vec![],
                command_inputs: vec![],
            }),
            config_root: Some(dir.to_path_buf()),
            ..Default::default()
        }
    }

    #[test]
    fn miss_run_record_then_hit() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.rs"), "1").unwrap();
        let task = task_in(dir.path(), "build");
        let cache = TaskRunCache::for_project(dir.path());
        let deps = BTreeMap::new();

        assert_eq!(cache.is_fresh(&task, &deps), Some(false));

        // Simulate the task producing its output, then record.
        fs::write(dir.path().join("out.txt"), "artifact").unwrap();
        cache.record(&task, &deps).unwrap();
        assert_eq!(cache.is_fresh(&task, &deps), Some(true));

        // Input change -> miss.
        fs::write(dir.path().join("a.rs"), "2").unwrap();
        assert_eq!(cache.is_fresh(&task, &deps), Some(false));
    }

    #[test]
    fn non_cacheable_task_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let mut task = task_in(dir.path(), "serve");
        task.cache = None;
        let cache = TaskRunCache::for_project(dir.path());
        assert_eq!(cache.is_fresh(&task, &BTreeMap::new()), None);
    }
}

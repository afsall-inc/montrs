// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic task hashing.
//!
//! A task's cache key is the content hash of everything that determines its
//! output: its command, working directory, declared input globs
//! ([`Task::sources`]), environment (both declared and an explicit allowlist),
//! tool versions, and the hashes of its dependencies.
//!
//! Two runs with identical inputs produce the same hash on any machine, so a
//! task whose inputs are unchanged is skipped.

use crate::types::{RunEntry, Task, TaskDep, TaskOutputs};
use montrs_cache::{CacheKey, Input};
use std::{collections::BTreeMap, path::Path};

/// Whether a task is cacheable, and the output globs that describe its effect.
///
/// Caching is opt-in: a task must set `cache = { enabled = true }` **and**
/// declare its `outputs`. A task whose outputs are left as `Auto` cannot be
/// verified, so it is never cached (it always runs).
pub fn cache_outputs(task: &Task) -> Option<Vec<String>> {
    let enabled = task.cache.as_ref().map(|c| c.enabled).unwrap_or(false);
    if !enabled {
        return None;
    }
    match &task.outputs {
        TaskOutputs::Files(files) => Some(files.clone()),
        TaskOutputs::NoFiles => Some(Vec::new()),
        // Cannot verify side effects -> never skip.
        TaskOutputs::Auto => None,
    }
}

/// Build the cache key for a task.
///
/// `dep_hashes` maps a dependency task name to its already-computed hash; when
/// a dependency's inputs change, the dependent task's key changes too.
pub fn task_key(
    task: &Task,
    dep_hashes: &BTreeMap<String, String>,
) -> Option<CacheKey> {
    let mut inputs = Vec::new();

    // Command identity.
    inputs.push(Input::value(format!("name={}", task.name)));
    for entry in &task.command {
        inputs.push(Input::value(command_repr(entry)));
    }
    if let Some(dir) = &task.dir {
        inputs.push(Input::value(format!("dir={dir}")));
    }
    if let Some(shell) = &task.shell {
        inputs.push(Input::value(format!("shell={shell}")));
    }
    for (k, v) in &task.env {
        inputs.push(Input::value(format!("env:{k}={v}")));
    }
    for (k, v) in &task.vars {
        inputs.push(Input::value(format!("var:{k}={v}")));
    }
    for (k, v) in &task.tools {
        let version = match v {
            crate::types::TaskToolValue::String(s) => s.clone(),
            crate::types::TaskToolValue::Map(m) => m.version.clone(),
        };
        inputs.push(Input::tool(k.clone(), version));
    }

    // Declared input globs, relative to the task root.
    let root = task_root(task);
    for pattern in &task.sources {
        inputs.push(Input::glob(&root, pattern));
    }

    // Environment allowlist + command inputs from the cache config.
    if let Some(cache) = &task.cache {
        for name in &cache.env {
            inputs.push(Input::env(name));
        }
        for value in &cache.command_inputs {
            inputs.push(Input::value(format!("input={value}")));
        }
    }

    // Dependency hashes.
    for dep in &task.depends {
        let name = dep.task_name();
        let hash = dep_hashes
            .get(name)
            .cloned()
            .unwrap_or_else(|| "<unhashed>".to_string());
        inputs.push(Input::value(format!("dep:{name}={hash}")));
    }

    CacheKey::new("task", &inputs).ok()
}

/// The directory a task's globs resolve from.
pub fn task_root(task: &Task) -> std::path::PathBuf {
    let base = task
        .config_root
        .clone()
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    match &task.dir {
        Some(dir) => base.join(dir),
        None => base,
    }
}

/// A stable string form of a run entry for hashing.
fn command_repr(entry: &RunEntry) -> String {
    match entry {
        RunEntry::Script(script) => format!("script:{script}"),
        RunEntry::SingleTask { task, args, env } => {
            format!("task:{task}:{args:?}:{env:?}")
        }
        RunEntry::TaskGroup { tasks } => format!("group:{tasks:?}"),
    }
}

/// Hash a raw command line (used for `--affected` comparisons).
pub fn hash_command(command: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(command.as_bytes());
    hex::encode(hasher.finalize())
}

/// A stable identifier for a dependency reference.
pub fn dep_name(dep: &TaskDep) -> &str {
    dep.task_name()
}

/// Resolve a list of output globs to concrete paths under `root`.
pub fn resolve_outputs(
    root: &Path,
    globs: &[String],
) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for pattern in globs {
        let joined = root.join(pattern);
        let joined = joined.to_string_lossy().replace('\\', "/");
        if let Ok(paths) = glob::glob(&joined) {
            for path in paths.filter_map(Result::ok) {
                out.push(path);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Task, TaskCacheConfig};
    use std::fs;

    fn cached_task(name: &str, dir: &Path, source: &str) -> Task {
        Task {
            name: name.to_string(),
            command: vec![RunEntry::Script("echo hi".to_string())],
            sources: vec![source.to_string()],
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
    fn identical_tasks_hash_equally() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.rs"), "1").unwrap();
        let a = cached_task("t", dir.path(), "**/*.rs");
        let b = cached_task("t", dir.path(), "**/*.rs");
        let hashes = BTreeMap::new();
        assert_eq!(task_key(&a, &hashes), task_key(&b, &hashes));
    }

    #[test]
    fn source_change_changes_hash() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.rs"), "1").unwrap();
        let task = cached_task("t", dir.path(), "**/*.rs");
        let first = task_key(&task, &BTreeMap::new()).unwrap();
        fs::write(dir.path().join("a.rs"), "2").unwrap();
        let second = task_key(&task, &BTreeMap::new()).unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn env_allowlist_changes_hash() {
        let dir = tempfile::tempdir().unwrap();
        let mut task = cached_task("t", dir.path(), "**/*.rs");
        task.cache = Some(TaskCacheConfig {
            enabled: true,
            audit: false,
            env: vec!["MONTRS_TEST_ENV".to_string()],
            command_inputs: vec![],
        });

        // SAFETY: single-threaded test.
        unsafe { std::env::set_var("MONTRS_TEST_ENV", "one") };
        let first = task_key(&task, &BTreeMap::new()).unwrap();
        unsafe { std::env::set_var("MONTRS_TEST_ENV", "two") };
        let second = task_key(&task, &BTreeMap::new()).unwrap();
        assert_ne!(first, second);
        unsafe { std::env::remove_var("MONTRS_TEST_ENV") };
    }

    #[test]
    fn dependency_hash_changes_key() {
        let dir = tempfile::tempdir().unwrap();
        let mut task = cached_task("t", dir.path(), "**/*.rs");
        task.depends = vec![crate::types::TaskDep::Simple("build".to_string())];

        let mut a = BTreeMap::new();
        a.insert("build".to_string(), "hash1".to_string());
        let mut b = BTreeMap::new();
        b.insert("build".to_string(), "hash2".to_string());

        assert_ne!(task_key(&task, &a), task_key(&task, &b));
    }

    #[test]
    fn caching_is_opt_in_and_requires_outputs() {
        let dir = tempfile::tempdir().unwrap();
        let mut task = cached_task("t", dir.path(), "**/*.rs");
        assert!(cache_outputs(&task).is_some());

        // Disabled -> not cacheable.
        task.cache = None;
        assert!(cache_outputs(&task).is_none());

        // Enabled but outputs Auto -> not cacheable.
        task.cache = Some(TaskCacheConfig {
            enabled: true,
            audit: false,
            env: vec![],
            command_inputs: vec![],
        });
        task.outputs = TaskOutputs::Auto;
        assert!(cache_outputs(&task).is_none());
    }
}

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # montrs-scm
//!
//! Minimal, deterministic git change detection.
//!
//! Shells out to the `git` binary (no heavyweight git library, no daemon) and
//! returns **sorted, repo-relative** paths with `/` separators so results are
//! stable across machines and useful for `--affected` task filtering.
//!
//! When a directory is not a git repository, `is_repo` returns `false` and
//! [`changed_files`] returns an empty list — callers should then run everything.

use montrs_core::AgentError;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use thiserror::Error;

/// Errors from git interaction.
#[derive(Debug, Error)]
pub enum ScmError {
    /// The `git` binary could not be executed.
    #[error("could not run git: {0}")]
    GitUnavailable(String),
    /// A git command failed.
    #[error("git {args} failed: {stderr}")]
    CommandFailed {
        /// The arguments passed to git.
        args: String,
        /// The stderr git produced.
        stderr: String,
    },
}

impl AgentError for ScmError {
    fn error_code(&self) -> &'static str {
        match self {
            ScmError::GitUnavailable(_) => "SCM_GIT_UNAVAILABLE",
            ScmError::CommandFailed { .. } => "SCM_COMMAND_FAILED",
        }
    }

    fn explanation(&self) -> String {
        match self {
            ScmError::GitUnavailable(e) => {
                format!("git could not be executed: {e}.")
            }
            ScmError::CommandFailed { args, stderr } => {
                format!("`git {args}` failed: {stderr}.")
            }
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        vec![
            "Ensure `git` is installed and on PATH.".to_string(),
            "Run the command inside a git repository.".to_string(),
        ]
    }

    fn subsystem(&self) -> &'static str {
        "scm"
    }
}

/// Whether `root` is inside a git work tree.
pub fn is_repo(root: &Path) -> bool {
    run_git(root, &["rev-parse", "--is-inside-work-tree"])
        .map(|out| out.trim() == "true")
        .unwrap_or(false)
}

/// The current `HEAD` revision, if any (`None` before the first commit).
pub fn head_rev(root: &Path) -> Option<String> {
    run_git(root, &["rev-parse", "HEAD"])
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Every file changed since `since` (a revision), plus uncommitted changes.
///
/// `since` may be `None`, in which case only uncommitted (working tree +
/// index) changes are returned. Results are sorted, deduped, repo-relative, and
/// use `/` separators.
pub fn changed_files(root: &Path, since: Option<&str>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    if let Some(since) = since {
        // Committed changes since the revision.
        let range = format!("{since}...HEAD");
        if let Ok(out) = run_git(
            root,
            &["diff", "--name-only", "--diff-filter=ACMR", &range],
        ) {
            files.extend(parse_paths(&out));
        }
    }

    // Uncommitted changes (staged + unstaged) and untracked files.
    if let Ok(out) = run_git(root, &["status", "--porcelain"]) {
        files.extend(parse_porcelain(&out));
    }

    files.sort();
    files.dedup();
    files
}

/// Whether a set of changed paths matches any of the given globs.
///
/// Globs are matched against the repo-relative path with `/` separators. An
/// empty glob list means "always matches" (run the task).
pub fn matches_any(changed: &[PathBuf], globs: &[String]) -> bool {
    if globs.is_empty() {
        return true;
    }
    changed.iter().any(|path| {
        let p = path.to_string_lossy().replace('\\', "/");
        globs.iter().any(|g| matches_glob(g, &p))
    })
}

/// A tiny, dependency-free glob matcher supporting `*`, `**`, and `?`.
fn matches_glob(pattern: &str, path: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = path.chars().collect();
    glob_match(&pat, &text)
}

fn glob_match(pat: &[char], text: &[char]) -> bool {
    if pat.is_empty() {
        return text.is_empty();
    }
    match pat[0] {
        // `**` matches any number of path segments (including `/`).
        '*' if pat.get(1) == Some(&'*') => {
            let rest = &pat[2..];
            // Skip a following `/` so `**/*.rs` also matches root files.
            let rest = if rest.first() == Some(&'/') {
                &rest[1..]
            } else {
                rest
            };
            for i in 0..=text.len() {
                if glob_match(rest, &text[i..]) {
                    return true;
                }
            }
            false
        }
        // `*` matches within a segment (no `/`).
        '*' => {
            let rest = &pat[1..];
            for i in 0..=text.len() {
                if glob_match(rest, &text[i..]) {
                    return true;
                }
                if i < text.len() && text[i] == '/' {
                    break;
                }
            }
            false
        }
        '?' => {
            !text.is_empty()
                && text[0] != '/'
                && glob_match(&pat[1..], &text[1..])
        }
        c => {
            !text.is_empty()
                && text[0] == c
                && glob_match(&pat[1..], &text[1..])
        }
    }
}

fn run_git(root: &Path, args: &[&str]) -> Result<String, ScmError> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| ScmError::GitUnavailable(e.to_string()))?;
    if !output.status.success() {
        return Err(ScmError::CommandFailed {
            args: args.join(" "),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn parse_paths(output: &str) -> Vec<PathBuf> {
    output
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(normalize)
        .collect()
}

/// Parse `git status --porcelain` output (v1) into changed paths.
fn parse_porcelain(output: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for line in output.lines() {
        if line.len() < 4 {
            continue;
        }
        // Format: `XY <path>` (with a rename arrow `old -> new`).
        let rest = &line[3..];
        let path = match rest.split_once(" -> ") {
            Some((_, new)) => new,
            None => rest,
        };
        out.push(normalize(path));
    }
    out
}

fn normalize(path: &str) -> PathBuf {
    PathBuf::from(path.trim().trim_matches('"').replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glob_semantics() {
        assert!(matches_glob("**/*.rs", "app/src/main.rs"));
        assert!(matches_glob("**/*.rs", "main.rs"));
        assert!(!matches_glob("**/*.rs", "app/src/main.ts"));
        assert!(matches_glob("src/*.rs", "src/a.rs"));
        assert!(!matches_glob("src/*.rs", "src/nested/a.rs"));
        assert!(matches_glob("*.toml", "Cargo.toml"));
        assert!(matches_glob("app/?ain.rs", "app/main.rs"));
    }

    #[test]
    fn matches_any_empty_globs_is_true() {
        assert!(matches_any(&[PathBuf::from("a.rs")], &[]));
    }

    #[test]
    fn matches_any_checks_each_path() {
        let changed = vec![PathBuf::from("packages/ui/src/button.rs")];
        assert!(matches_any(&changed, &["packages/ui/**/*.rs".to_string()]));
        assert!(!matches_any(
            &changed,
            &["apps/website/**/*.rs".to_string()]
        ));
    }

    #[test]
    fn parses_porcelain_status() {
        let output = " M apps/website/app/src/lib.rs\n?? new-file.rs\nR  \
                      old.rs -> new.rs\n";
        let paths = parse_porcelain(output);
        assert_eq!(
            paths,
            vec![
                PathBuf::from("apps/website/app/src/lib.rs"),
                PathBuf::from("new-file.rs"),
                PathBuf::from("new.rs"),
            ]
        );
    }

    #[test]
    fn non_repo_is_detected_and_empty() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!is_repo(dir.path()));
        assert!(changed_files(dir.path(), Some("HEAD")).is_empty());
    }
}

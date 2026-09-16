// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Template CI guard.
//!
//! The templates are shipped to users, so they must keep compiling against the
//! current framework APIs. This test typechecks the in-repo-buildable templates
//! (`default`, `todo`, `monorepo`) so drift is caught in CI instead of by a
//! user running `montrs new`.
//!
//! It compiles whole applications, so it is `#[ignore]`d for the normal test
//! run and enabled explicitly in CI:
//!
//! ```text
//! cargo test -p montrs-cli --test template_check -- --ignored --nocapture
//! ```
//!
//! Templates that still contain `{{project-name}}`/`{{crate_name}}` placeholders
//! (`saas`, `api`, `desktop`) are not typechecked here: they are only valid once
//! `montrs new` substitutes them into the destination project.

use std::{path::PathBuf, process::Command};

/// `(template, package, extra cargo args)` to check.
///
/// The app packages are checked explicitly rather than the whole workspace so a
/// template's `e2e` member (which needs the Playwright driver) is not compiled.
const CHECKS: &[(&str, &str, &[&str])] = &[
    ("default", "app", &["--features", "ssr"]),
    ("todo", "app", &["--features", "ssr"]),
    ("todo", "todo-example", &[]),
    ("monorepo", "web", &["--features", "ssr"]),
];

fn repo_root() -> PathBuf {
    // .../packages/cli -> .../packages -> repo root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .to_path_buf()
}

#[test]
#[ignore = "compiles whole apps; run explicitly in CI"]
fn buildable_templates_typecheck() {
    let root = repo_root();
    // Share one target dir across templates so their common dependencies are
    // compiled once.
    let target_dir = root.join("target").join("template-check");

    for (template, package, extra) in CHECKS {
        let manifest = root.join("templates").join(template).join("Cargo.toml");
        assert!(
            manifest.is_file(),
            "template `{template}` has no Cargo.toml at {}",
            manifest.display()
        );

        let mut cmd = Command::new(env!("CARGO"));
        cmd.arg("check")
            .arg("--manifest-path")
            .arg(&manifest)
            .arg("--package")
            .arg(package)
            .args(*extra)
            .env("CARGO_TARGET_DIR", &target_dir);

        let status = cmd
            .status()
            .unwrap_or_else(|e| panic!("failed to run cargo for `{template}`: {e}"));

        assert!(
            status.success(),
            "template `{template}` package `{package}` failed to check"
        );
    }
}

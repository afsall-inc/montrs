// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Linker shim used by `montrs serve --hotpatch`.
//!
//! Set as `CARGO_TARGET_<TRIPLE>_LINKER`, this:
//! * in fat-link mode, and only for the tip link (its `/OUT:` matches
//!   `MONTRS_HOTPATCH_TIP_OUT`), archives the workspace rlibs and relinks with
//!   the hot-patch flags — this must happen here, while rustc's temporary
//!   objects still exist;
//! * otherwise records the link invocation (when `MONTRS_HOTPATCH_DIR` is set)
//!   and forwards to the real linker named by `MONTRS_REAL_LINKER`.

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

fn real_linker() -> PathBuf {
    std::env::var_os("MONTRS_REAL_LINKER")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if cfg!(windows) {
                PathBuf::from("link.exe")
            } else {
                PathBuf::from("cc")
            }
        })
}

fn flavor() -> montrs_dev_hotpatch::LinkerFlavor {
    match std::env::var("MONTRS_HOTPATCH_FLAVOR").as_deref() {
        Ok("msvc") => montrs_dev_hotpatch::LinkerFlavor::Msvc,
        Ok("gnu") => montrs_dev_hotpatch::LinkerFlavor::Gnu,
        _ => montrs_dev_hotpatch::flavor_from_triple(
            &std::env::var("HOST").unwrap_or_default(),
        ),
    }
}

/// Normalize a file stem for comparison: fold case and treat `-`/`_` as the
/// same separator, so `website-ssr` matches `website_ssr`.
fn normalize_stem(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().to_ascii_lowercase().replace('-', "_"))
        .unwrap_or_default()
}

/// Whether this invocation produces the tip binary the hot-patch target cares
/// about.
///
/// Cargo links the binary to `deps/<crate_name>.exe` (underscored, sometimes
/// hashed) and then hardlinks the dash-named path, so we compare file names,
/// not full paths.
fn is_tip(args: &[String]) -> bool {
    let Some(expected) = std::env::var_os("MONTRS_HOTPATCH_TIP_OUT") else {
        return false;
    };
    let expected = normalize_stem(Path::new(&expected));
    let Some(out) = montrs_dev_hotpatch::link_output(args) else {
        return false;
    };
    let actual = normalize_stem(&out);
    actual == expected
        || actual
            .strip_prefix(&expected)
            .is_some_and(|rest| rest.starts_with('_'))
}

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let raw: Vec<String> = args
        .iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    // rustc hides long commands behind `@response-file`s; expand for inspection.
    let resolved = montrs_dev_hotpatch::expand_response_args(&raw);
    let real = real_linker();

    if std::env::var_os("MONTRS_HOTPATCH_FATLINK").is_some()
        && is_tip(&resolved)
        && let Some(workspace) =
            std::env::var_os("MONTRS_HOTPATCH_WORKSPACE")
    {
        let archive = montrs_dev_hotpatch::capture_dir().join("fat.a");
        match montrs_dev_hotpatch::fat_link_in_place(
            &real,
            flavor(),
            &resolved,
            Path::new(&workspace),
            &archive,
        ) {
            Ok(count) => {
                eprintln!(
                    "montrs: fat link complete ({count} workspace objects \
                     archived)."
                );
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("montrs: fat link failed: {e}");
                return ExitCode::from(1);
            }
        }
    }

    if std::env::var_os("MONTRS_HOTPATCH_DIR").is_some() {
        let invocation = montrs_dev_hotpatch::LinkInvocation {
            args: resolved.clone(),
            cwd: std::env::current_dir().unwrap_or_default(),
        };
        let _ = montrs_dev_hotpatch::capture_link(&invocation);
    }

    match Command::new(&real).args(&args).status() {
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(e) => {
            eprintln!(
                "montrs-link-wrapper: failed to run {}: {e}",
                real.display()
            );
            ExitCode::from(1)
        }
    }
}

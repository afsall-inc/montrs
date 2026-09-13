// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Linker shim used by `montrs serve --hotpatch`.
//!
//! Set as `CARGO_TARGET_<TRIPLE>_LINKER`, this records the link invocation
//! (when `MONTRS_HOTPATCH_DIR` is set) and forwards to the real linker named
//! by `MONTRS_REAL_LINKER`, so a later pass can thin-link a patch against the
//! running binary's symbols.

use std::{
    ffi::OsString,
    path::PathBuf,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();

    if std::env::var_os("MONTRS_HOTPATCH_DIR").is_some() {
        let invocation = montrs_dev_hotpatch::LinkInvocation {
            args: args
                .iter()
                .map(|a| a.to_string_lossy().into_owned())
                .collect(),
            cwd: std::env::current_dir().unwrap_or_default(),
        };
        // Best-effort: capture must never break the link.
        let _ = montrs_dev_hotpatch::capture_link(&invocation);
    }

    let real = std::env::var_os("MONTRS_REAL_LINKER")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if cfg!(windows) {
                PathBuf::from("link.exe")
            } else {
                PathBuf::from("cc")
            }
        });

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

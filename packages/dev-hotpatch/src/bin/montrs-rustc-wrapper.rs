// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `RUSTC_WORKSPACE_WRAPPER` shim used by `montrs serve --hotpatch`.
//!
//! Cargo invokes this as `<wrapper> <real-rustc> <args…>`. We record the
//! invocation (when `MONTRS_HOTPATCH_DIR` is set) so a later patch pass can
//! replay only the changed workspace crates, then forward to the real rustc
//! unchanged.

use std::{
    ffi::OsString,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let mut args = std::env::args_os();
    let _wrapper = args.next();
    let Some(real_rustc) = args.next() else {
        eprintln!("montrs-rustc-wrapper: missing rustc path");
        return ExitCode::from(2);
    };
    let rest: Vec<OsString> = args.collect();

    if std::env::var_os("MONTRS_HOTPATCH_DIR").is_some() {
        let invocation = montrs_dev_hotpatch::RustcInvocation {
            args: std::iter::once(real_rustc.to_string_lossy().into_owned())
                .chain(rest.iter().map(|a| a.to_string_lossy().into_owned()))
                .collect(),
            cwd: std::env::current_dir().unwrap_or_default(),
            envs: montrs_dev_hotpatch::capture_env_allowlist(),
        };
        // Capture is best-effort: a failure must never break the build.
        let _ = montrs_dev_hotpatch::capture_rustc(&invocation);
    }

    match Command::new(&real_rustc).args(&rest).status() {
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(e) => {
            eprintln!("montrs-rustc-wrapper: failed to run rustc: {e}");
            ExitCode::from(1)
        }
    }
}

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Development helper: relink a built server as a hot-patchable fat binary
//! using the most recent captured link invocation.
//!
//! Usage: `montrs-hotpatch-relink <exe> <workspace-target-dir> <fat-archive> <real-linker> <msvc|gnu>`

use std::{path::PathBuf, process::ExitCode};

use montrs_dev_hotpatch::LinkerFlavor;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 {
        eprintln!(
            "usage: montrs-hotpatch-relink <exe> <workspace-target-dir> \
             <fat-archive> <real-linker> <msvc|gnu>"
        );
        return ExitCode::from(2);
    }
    let exe = PathBuf::from(&args[1]);
    let target = PathBuf::from(&args[2]);
    let archive = PathBuf::from(&args[3]);
    let linker = PathBuf::from(&args[4]);
    let flavor = match args[5].as_str() {
        "msvc" => LinkerFlavor::Msvc,
        "gnu" => LinkerFlavor::Gnu,
        _ => LinkerFlavor::Other,
    };

    let Some(link) = montrs_dev_hotpatch::read_latest_link() else {
        eprintln!("no captured link invocation found");
        return ExitCode::from(1);
    };

    match montrs_dev_hotpatch::relink_fat(
        &linker,
        flavor,
        &link.args,
        &target,
        &archive,
        &exe,
    ) {
        Ok(count) => {
            println!("fat relink complete ({count} workspace objects archived)");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

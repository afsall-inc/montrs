// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Development helper: build a patch from captured tip objects.
//!
//! Usage: `montrs-hotpatch-build <exe> <capture-base> <real-linker> <msvc|gnu> [aslr-hex]`

use std::{path::Path, process::ExitCode};

use montrs_dev_hotpatch::{LinkerFlavor, PatchRequest};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "usage: montrs-hotpatch-build <exe> <capture-base> \
             <real-linker> <msvc|gnu> [aslr-hex]"
        );
        return ExitCode::from(2);
    }
    let exe = Path::new(&args[1]);
    let capture = Path::new(&args[2]);
    let linker = Path::new(&args[3]);
    let flavor = match args[4].as_str() {
        "msvc" => LinkerFlavor::Msvc,
        "gnu" => LinkerFlavor::Gnu,
        _ => LinkerFlavor::Other,
    };
    let aslr_reference = args
        .get(5)
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0);

    let request = PatchRequest {
        capture_base: capture,
        exe,
        real_linker: linker,
        flavor,
        aslr_reference,
        build_id: 0,
        pid: Some(std::process::id()),
    };
    match montrs_dev_hotpatch::build_patch(&request) {
        Ok(table) => {
            println!(
                "patch built: {} ({} jump-table entries)",
                table.lib.display(),
                table.map.len()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

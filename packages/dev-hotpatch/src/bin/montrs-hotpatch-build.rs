// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Development helper: build a patch from captured tip objects.
//!
//! Usage: `montrs-hotpatch-build <exe> <capture-base> <real-linker> <msvc|gnu> <workspace-target> [aslr-hex]`

use std::{path::Path, process::ExitCode};

use montrs_dev_hotpatch::{LinkerFlavor, PatchRequest, SymbolIndex};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    // Debug aid: `--find <exe> <symbol>` prints whether the fat index has it.
    if args.len() >= 4 && args[1] == "--find" {
        let exe = Path::new(&args[2]);
        let name = &args[3];
        match SymbolIndex::from_exe(exe) {
            Ok(index) => {
                println!("symbols indexed: {}", index.len());
                match index.symbol(name) {
                    Some(s) => println!(
                        "{name}: addr={:#x} kind={:?} undefined={}",
                        s.address, s.kind, s.is_undefined
                    ),
                    None => println!("{name}: MISSING"),
                }
            }
            Err(e) => eprintln!("error: {e}"),
        }
        return ExitCode::SUCCESS;
    }

    // Debug aid: `--list <exe> <substr>` lists symbols whose name contains it.
    if args.len() >= 4 && args[1] == "--list" {
        let exe = Path::new(&args[2]);
        let needle = &args[3];
        match SymbolIndex::from_exe(exe) {
            Ok(index) => {
                let mut n = 0usize;
                for (name, s) in index.iter() {
                    if name.contains(needle.as_str()) {
                        println!("{:#x} {:?} {}", s.address, s.kind, name);
                        n += 1;
                    }
                }
                println!("matched {n} / {}", index.len());
            }
            Err(e) => eprintln!("error: {e}"),
        }
        return ExitCode::SUCCESS;
    }

    if args.len() < 6 {
        eprintln!(
            "usage: montrs-hotpatch-build <exe> <capture-base> \
             <real-linker> <msvc|gnu> <workspace-target> [aslr-hex]"
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
    let workspace_target_dir = Path::new(&args[5]);
    let aslr_reference = args
        .get(6)
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0);

    let request = PatchRequest {
        capture_base: capture,
        exe,
        workspace_target_dir,
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

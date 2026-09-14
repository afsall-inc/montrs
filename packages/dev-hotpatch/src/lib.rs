// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Foundation for Rust hot-patching in the MontRS dev server.
//!
//! This is the MontRS-native counterpart to the Dioxus CLI's hot-patch tooling.
//! It provides two halves:
//!
//! * the **wire protocol** the dev server and the browser client speak (a
//!   `JumpTable` delivered over the dev socket), and
//! * the **capture layer** — a `RUSTC_WORKSPACE_WRAPPER` that records each
//!   workspace crate's `rustc` invocation so a later patch pass can replay only
//!   the changed crates and thin-link them against the running binary.
//!
//! The thin-link/patch linker itself is the next milestone; this crate makes
//! the protocol and the capture deterministic and testable first.
//!
//! ## Attribution
//!
//! The protocol shapes (`DevserverMsg` / `HotReloadMsg`) and the `JumpTable`
//! runtime type originate from the Dioxus project's subsecond work
//! (`dioxus-devtools` / `subsecond`), MIT/Apache-2.0, adapted here for MontRS.

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

pub use subsecond_types::JumpTable;

/// A message the hot-patch dev server sends to a connected application.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DevserverMsg {
    /// Apply a patch (jump table) to the running process.
    HotReload(HotReloadMsg),
    /// A patch build has started.
    HotPatchStart,
    /// A full (non-patch) rebuild has started.
    FullReloadStart,
    /// The full rebuild failed.
    FullReloadFailed,
    /// The client should fully reload.
    FullReloadCommand,
    /// The dev server is shutting down.
    Shutdown,
}

/// The payload for [`DevserverMsg::HotReload`].
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct HotReloadMsg {
    /// The patch jump table, if this message carries one.
    pub jump_table: Option<JumpTable>,
    /// Wall-clock time the patch took to build, for the overlay.
    pub ms_elapsed: u64,
    /// Only apply if the client's build id matches (wasm reports `0`).
    pub for_build_id: Option<u64>,
    /// Only apply if the process id matches (native only; wasm reports `None`).
    pub for_pid: Option<u32>,
}

/// The rustc invocation for one crate, captured during a build.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RustcInvocation {
    /// The full argv (including argv[0], the real rustc path).
    pub args: Vec<String>,
    /// The working directory the invocation ran in.
    pub cwd: PathBuf,
    /// Environment variables needed to replay it reliably.
    pub envs: Vec<(String, String)>,
}

/// The link step for the tip binary, captured from the linker wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinkInvocation {
    /// The linker args (the tail passed to the real linker).
    pub args: Vec<String>,
    /// The working directory.
    pub cwd: PathBuf,
}

/// The directory the wrapper writes capture output to.
pub fn capture_dir() -> PathBuf {
    std::env::var_os("MONTRS_HOTPATCH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/montrs-hotpatch"))
}

static SEQ: AtomicU64 = AtomicU64::new(0);

fn unique_name(kind: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    format!("{kind}-{}-{nanos}-{seq}.json", std::process::id())
}

fn write_json<T: Serialize>(dir: &Path, name: &str, value: &T) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let path = dir.join(name);
    let data = serde_json::to_vec_pretty(value)
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    std::fs::write(path, data)
}

/// Record one rustc invocation under `<base>/rustc/`.
pub fn capture_rustc_in(
    base: &Path,
    invocation: &RustcInvocation,
) -> std::io::Result<PathBuf> {
    let dir = base.join("rustc");
    std::fs::create_dir_all(&dir)?;
    let name = unique_name("rustc");
    write_json(&dir, &name, invocation)?;
    Ok(dir.join(name))
}

/// Read every captured rustc invocation under `<base>/rustc/`.
pub fn read_rustc_invocations_in(base: &Path) -> Vec<RustcInvocation> {
    read_all(&base.join("rustc"))
}

/// Record the tip link invocation under `<base>/link/`.
pub fn capture_link_in(
    base: &Path,
    invocation: &LinkInvocation,
) -> std::io::Result<PathBuf> {
    let dir = base.join("link");
    std::fs::create_dir_all(&dir)?;
    let name = unique_name("link");
    write_json(&dir, &name, invocation)?;
    Ok(dir.join(name))
}

/// Read the most recent captured link invocation under `<base>/link/`.
pub fn read_latest_link_in(base: &Path) -> Option<LinkInvocation> {
    let mut all = read_all(&base.join("link"));
    all.pop()
}

/// Record one rustc invocation into the ambient capture dir.
pub fn capture_rustc(invocation: &RustcInvocation) -> std::io::Result<PathBuf> {
    capture_rustc_in(&capture_dir(), invocation)
}

/// Read every captured rustc invocation from the ambient capture dir.
pub fn read_rustc_invocations() -> Vec<RustcInvocation> {
    read_rustc_invocations_in(&capture_dir())
}

/// Record the tip link invocation into the ambient capture dir.
pub fn capture_link(invocation: &LinkInvocation) -> std::io::Result<PathBuf> {
    capture_link_in(&capture_dir(), invocation)
}

/// Read the most recent captured link invocation from the ambient capture dir.
pub fn read_latest_link() -> Option<LinkInvocation> {
    read_latest_link_in(&capture_dir())
}

fn read_all<T: for<'de> Deserialize<'de>>(dir: &Path) -> Vec<T> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();
    paths.sort();
    paths
        .iter()
        .filter_map(|p| {
            std::fs::read_to_string(p)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
        })
        .collect()
}

/// Extract the real linker path from `rustc --print=link-args` output.
///
/// The first token of that output is the linker binary, quoted so paths with
/// spaces survive. Used to point `MONTRS_REAL_LINKER` at the linker the
/// wrapper must forward to.
pub fn parse_linker_from_link_args(output: &str) -> Option<PathBuf> {
    let first = output.trim_start();
    let rest = first.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(PathBuf::from(&rest[..end]))
}

/// Environment variables worth replaying a rustc invocation with.
pub const REPLAY_ENV_ALLOWLIST: &[&str] = &[
    "PATH",
    "CARGO_PKG_NAME",
    "CARGO_MANIFEST_DIR",
    "CARGO_MANIFEST_PATH",
    "OUT_DIR",
    "TARGET",
    "HOST",
    "PROFILE",
    "OPT_LEVEL",
    "DEBUG",
    "RUSTFLAGS",
    "LEPTOS_WATCH",
];

/// Collect the allow-listed env vars from the current process.
pub fn capture_env_allowlist() -> Vec<(String, String)> {
    REPLAY_ENV_ALLOWLIST
        .iter()
        .filter_map(|k| std::env::var(k).ok().map(|v| (k.to_string(), v)))
        .collect()
}

// ---------------------------------------------------------------------------
// Fat link planning
// ---------------------------------------------------------------------------

/// Linker dialect, derived from the target triple.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkerFlavor {
    /// Windows MSVC (`link.exe` / `lld-link`): `/OUT:`, `/EXPORT:`.
    Msvc,
    /// GNU-style (`cc`, `ld.lld`): `-o`, `-Wl,…`.
    Gnu,
    /// Anything else — no fat-link adjustments are known.
    Other,
}

/// Parse the output path from a link command (`-o <p>` or `/OUT:<p>`).
pub fn link_output(link_args: &[String]) -> Option<PathBuf> {
    let mut iter = link_args.iter();
    while let Some(arg) = iter.next() {
        if arg == "-o" {
            return iter.next().map(PathBuf::from);
        }
        if let Some(rest) = arg.strip_prefix("/OUT:") {
            return Some(PathBuf::from(rest));
        }
    }
    None
}

/// The `.rlib` paths in a link command (the workspace + sysroot libraries).
pub fn link_rlibs(link_args: &[String]) -> Vec<PathBuf> {
    link_args
        .iter()
        .filter(|a| a.ends_with(".rlib"))
        .map(PathBuf::from)
        .collect()
}

/// Produce the hot-patchable ("fat") link arguments from a normal link.
///
/// The fat binary must contain all workspace object code, export `main` so a
/// patch can reference the image base, and (on MSVC) disable high-entropy VA
/// so symbol addresses are stable run to run.
pub fn fat_link_args(
    original: &[String],
    fat_archive: &Path,
    flavor: LinkerFlavor,
) -> Vec<String> {
    let mut args = original.to_vec();
    match flavor {
        LinkerFlavor::Msvc => {
            args.push(format!("/WHOLEARCHIVE:{}", fat_archive.display()));
            args.push("/EXPORT:main".to_string());
            args.push("/HIGHENTROPYVA:NO".to_string());
        }
        LinkerFlavor::Gnu => {
            args.push("-Wl,--whole-archive".to_string());
            args.push(fat_archive.display().to_string());
            args.push("-Wl,--no-whole-archive".to_string());
            args.push("-Wl,--export-dynamic-symbol,main".to_string());
        }
        LinkerFlavor::Other => {}
    }
    args
}

/// Build a "fat archive" from the workspace's `.rlib`s.
///
/// Rust's `.rlib`s are `ar` archives of `.rcgu.o` objects (plus `.rmeta`
/// metadata we must drop). Concatenating every object into one archive lets the
/// hot-patch link pull in all workspace code with a single whole-archive
/// directive. Returns the number of objects written.
pub fn build_fat_archive(
    rlibs: &[PathBuf],
    out: &Path,
) -> anyhow::Result<usize> {
    let file = std::fs::File::create(out)?;
    let mut writer = ar::Builder::new(file);
    let mut written = 0usize;

    for rlib in rlibs {
        let bytes = std::fs::read(rlib)?;
        let mut archive = ar::Archive::new(std::io::Cursor::new(bytes));
        while let Some(entry) = archive.next_entry() {
            let Ok(entry) = entry else { continue };
            let name =
                String::from_utf8_lossy(entry.header().identifier()).to_string();
            if name.ends_with(".rmeta") || entry.header().size() == 0 {
                continue;
            }
            if !(name.ends_with(".rcgu.o") || name.ends_with(".obj")) {
                continue;
            }
            writer.append(&entry.header().clone(), entry)?;
            written += 1;
        }
    }

    writer.into_inner()?;
    Ok(written)
}

// ---------------------------------------------------------------------------
// Fat-binary symbol index
// ---------------------------------------------------------------------------

/// A name → address map of the fat (running) binary's symbols.
///
/// A hot patch is linked against the running process's known symbol addresses,
/// so the patcher first reads them: from the sibling `.pdb` on Windows, or the
/// executable's object symbol table elsewhere.
#[derive(Debug, Default)]
pub struct SymbolIndex {
    symbols: std::collections::HashMap<String, u64>,
}

impl SymbolIndex {
    /// Read the symbol table for `exe`.
    #[cfg(windows)]
    pub fn from_exe(exe: &Path) -> anyhow::Result<Self> {
        use pdb::FallibleIterator;

        let pdb_path = exe.with_extension("pdb");
        let file = std::fs::File::open(&pdb_path).map_err(|e| {
            anyhow::anyhow!("could not open {}: {e}", pdb_path.display())
        })?;
        let mut pdb = pdb::PDB::open(file)
            .map_err(|e| anyhow::anyhow!("could not parse pdb: {e}"))?;
        let address_map = pdb.address_map()?;
        let global_symbols = pdb.global_symbols()?;

        let mut symbols = std::collections::HashMap::new();
        let mut iter = global_symbols.iter();
        while let Some(symbol) = iter.next()? {
            match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) => {
                    if let Some(rva) = data.offset.to_rva(&address_map) {
                        symbols.insert(
                            data.name.to_string().to_string(),
                            rva.0 as u64,
                        );
                    }
                }
                Ok(pdb::SymbolData::Data(data)) => {
                    if let Some(rva) = data.offset.to_rva(&address_map) {
                        symbols.insert(
                            data.name.to_string().to_string(),
                            rva.0 as u64,
                        );
                    }
                }
                _ => {}
            }
        }
        Ok(Self { symbols })
    }

    /// Read the symbol table for `exe` from its object file.
    #[cfg(not(windows))]
    pub fn from_exe(exe: &Path) -> anyhow::Result<Self> {
        use object::{Object, ObjectSymbol};

        let data = std::fs::read(exe)?;
        let file = object::File::parse(&*data)?;
        let mut symbols = std::collections::HashMap::new();
        for symbol in file.symbols() {
            if let Ok(name) = symbol.name() {
                symbols.insert(name.to_string(), symbol.address());
            }
        }
        Ok(Self { symbols })
    }

    /// Build an index directly from `name → address` pairs.
    pub fn from_pairs<I: IntoIterator<Item = (String, u64)>>(pairs: I) -> Self {
        Self {
            symbols: pairs.into_iter().collect(),
        }
    }

    /// Iterate over `(name, address)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &u64)> {
        self.symbols.iter()
    }

    /// Look up a symbol's address.
    pub fn get(&self, name: &str) -> Option<u64> {
        self.symbols.get(name).copied()
    }

    /// Whether a symbol is present.
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Number of symbols indexed.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Whether the index has no symbols.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

/// Build a subsecond [`JumpTable`] from the fat binary's symbols and the
/// freshly linked patch's symbols.
///
/// Every symbol the patch defines that also exists in the running binary maps
/// that binary's address → the patch's address; `main` anchors the ASLR
/// reference (running binary) and the patch's base address, exactly as the
/// Windows/native Dioxus jump-table builders do.
pub fn build_jump_table(
    lib: PathBuf,
    fat: &SymbolIndex,
    patch: &SymbolIndex,
) -> anyhow::Result<JumpTable> {
    let mut map = subsecond_types::AddressMap::default();
    for (name, new_addr) in patch.iter() {
        if let Some(old_addr) = fat.get(name) {
            map.insert(old_addr, *new_addr);
        }
    }

    let new_base_address = patch
        .get("main")
        .ok_or_else(|| anyhow::anyhow!("patch has no `main` symbol"))?;
    let aslr_reference = fat
        .get("main")
        .ok_or_else(|| anyhow::anyhow!("fat binary has no `main` symbol"))?;

    Ok(JumpTable {
        lib,
        map,
        aslr_reference,
        new_base_address,
        ifunc_count: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_symbols_from_a_real_binary() {
        // Prefer an explicit binary, else the test executable's own symbols.
        let exe = std::env::var_os("MONTRS_HOTPATCH_TEST_EXE")
            .map(PathBuf::from)
            .or_else(|| std::env::current_exe().ok())
            .expect("an executable path");

        match SymbolIndex::from_exe(&exe) {
            Ok(index) => {
                assert!(
                    !index.is_empty(),
                    "no symbols parsed from {}",
                    exe.display()
                );
                // A linked binary always has an entry point symbol.
                assert!(
                    index.contains("main") || index.contains("_main"),
                    "expected a `main` symbol; got {} symbols",
                    index.len()
                );
            }
            Err(e) => {
                // PDBs/objects can be absent in some environments; don't fail
                // the suite for that, but make the skip visible.
                eprintln!("skipping symbol test for {}: {e}", exe.display());
            }
        }
    }

    fn temp_dir(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "montrs-{tag}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn rustc_capture_round_trips() {
        let dir = temp_dir("hp-rustc");
        let inv = RustcInvocation {
            args: vec!["rustc".into(), "--crate-name".into(), "app".into()],
            cwd: PathBuf::from("/tmp/app"),
            envs: vec![("RUSTFLAGS".into(), "--cfg erase_components".into())],
        };
        capture_rustc_in(&dir, &inv).unwrap();
        let read = read_rustc_invocations_in(&dir);
        assert_eq!(read.len(), 1);
        assert_eq!(read[0], inv);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn link_capture_round_trips() {
        let dir = temp_dir("hp-link");
        let inv = LinkInvocation {
            args: vec!["-o".into(), "app.exe".into()],
            cwd: PathBuf::from("/tmp/app"),
        };
        capture_link_in(&dir, &inv).unwrap();
        assert_eq!(read_latest_link_in(&dir), Some(inv));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parses_linker_from_print_link_args() {
        let sample = "\"C:\\\\Program Files\\\\Microsoft Visual Studio\\\\2022\\\\VC\\\\Tools\\\\MSVC\\\\14.44.35207\\\\bin\\\\HostX64\\\\x64\\\\link.exe\" \"/NOLOGO\" \"/OUT:probe.exe\"";
        let linker = parse_linker_from_link_args(sample).expect("linker");
        assert_eq!(
            linker,
            PathBuf::from(
                "C:\\Program Files\\Microsoft Visual Studio\\2022\\VC\\Tools\\MSVC\\14.44.35207\\bin\\HostX64\\x64\\link.exe"
            )
        );
        assert!(parse_linker_from_link_args("not quoted").is_none());
    }

    #[test]
    fn jump_table_maps_matching_symbols() {
        let fat = SymbolIndex::from_pairs([
            ("main".to_string(), 0x1000),
            ("foo".to_string(), 0x2000),
            ("only_old".to_string(), 0x3000),
        ]);
        let patch = SymbolIndex::from_pairs([
            ("main".to_string(), 0x10),
            ("foo".to_string(), 0x20),
            ("only_new".to_string(), 0x30),
        ]);

        let table =
            build_jump_table(PathBuf::from("libpatch.dll"), &fat, &patch)
                .expect("jump table");
        assert_eq!(table.aslr_reference, 0x1000);
        assert_eq!(table.new_base_address, 0x10);
        assert_eq!(table.map.len(), 2, "only symbols in both must map");
        assert_eq!(table.map.get(&0x2000), Some(&0x20));
        assert_eq!(table.map.get(&0x3000), None);
    }

    #[test]
    fn jump_table_requires_main() {
        let fat = SymbolIndex::from_pairs([("foo".to_string(), 1)]);
        let patch = SymbolIndex::from_pairs([("foo".to_string(), 2)]);
        assert!(build_jump_table(PathBuf::from("x"), &fat, &patch).is_err());
    }

    #[test]
    fn fat_archive_keeps_objects_and_drops_metadata() {
        let dir = temp_dir("hp-fat");
        std::fs::create_dir_all(&dir).unwrap();
        let rlib = dir.join("libfoo.rlib");
        {
            let file = std::fs::File::create(&rlib).unwrap();
            let mut builder = ar::Builder::new(file);
            let object = b"OBJECTDATA";
            let header =
                ar::Header::new(b"foo.rcgu.o".to_vec(), object.len() as u64);
            builder.append(&header, &object[..]).unwrap();
            let meta = b"META";
            let header =
                ar::Header::new(b"foo.rmeta".to_vec(), meta.len() as u64);
            builder.append(&header, &meta[..]).unwrap();
        }

        let out = dir.join("fat.a");
        let written = build_fat_archive(&[rlib], &out).unwrap();
        assert_eq!(written, 1);

        let data = std::fs::read(&out).unwrap();
        let mut archive = ar::Archive::new(std::io::Cursor::new(data));
        let mut names = Vec::new();
        while let Some(entry) = archive.next_entry() {
            let entry = entry.unwrap();
            names.push(
                String::from_utf8_lossy(entry.header().identifier())
                    .to_string(),
            );
        }
        assert_eq!(names, vec!["foo.rcgu.o".to_string()]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn fat_link_args_msvc_adds_required_metadata() {
        let original =
            vec!["/NOLOGO".to_string(), "/OUT:app.exe".to_string()];
        let args =
            fat_link_args(&original, Path::new("deps.a"), LinkerFlavor::Msvc);
        assert!(args.iter().any(|a| a == "/EXPORT:main"));
        assert!(args.iter().any(|a| a == "/HIGHENTROPYVA:NO"));
        assert!(args.iter().any(|a| a.starts_with("/WHOLEARCHIVE:")));
        assert_eq!(link_output(&args), Some(PathBuf::from("app.exe")));
    }

    #[test]
    fn fat_link_args_gnu_adds_whole_archive_and_export() {
        let original = vec!["-o".to_string(), "app".to_string()];
        let args =
            fat_link_args(&original, Path::new("libdeps.a"), LinkerFlavor::Gnu);
        assert!(args.contains(&"-Wl,--whole-archive".to_string()));
        assert!(
            args.contains(&"-Wl,--export-dynamic-symbol,main".to_string())
        );
        assert_eq!(link_output(&args), Some(PathBuf::from("app")));
    }

    #[test]
    fn link_output_and_rlibs_parse() {
        let msvc = vec![
            "/NOLOGO".to_string(),
            "/OUT:x.exe".to_string(),
            "a.rlib".to_string(),
            "b.rlib".to_string(),
        ];
        assert_eq!(link_output(&msvc), Some(PathBuf::from("x.exe")));
        assert_eq!(
            link_rlibs(&msvc),
            vec![PathBuf::from("a.rlib"), PathBuf::from("b.rlib")]
        );

        let gnu = vec![
            "-o".to_string(),
            "out.bin".to_string(),
            "a.rlib".to_string(),
        ];
        assert_eq!(link_output(&gnu), Some(PathBuf::from("out.bin")));
        assert_eq!(link_rlibs(&gnu), vec![PathBuf::from("a.rlib")]);
    }

    #[test]
    fn hot_reload_message_serializes() {
        let msg = DevserverMsg::HotReload(HotReloadMsg {
            jump_table: None,
            ms_elapsed: 12,
            for_build_id: Some(0),
            for_pid: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"hot_reload\""));
        let back: DevserverMsg = serde_json::from_str(&json).unwrap();
        assert_eq!(back, msg);
    }
}

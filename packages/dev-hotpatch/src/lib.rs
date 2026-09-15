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

use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

pub mod client;
pub mod server;

pub use subsecond_types::JumpTable;

/// A message the hot-patch dev server sends to a connected application.
///
/// Externally tagged (like `dioxus_debugtools::DevserverMsg`), so it serializes
/// as `{"HotReload":{…}}`. This matters: Serde's internally-tagged
/// representation buffers values and cannot deserialize a populated
/// `subsecond_types::AddressMap` (its `u64` keys arrive as JSON strings),
/// whereas the external form round-trips. Leptos's
/// `connect_to_hot_patch_messages` parses this same shape.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

/// A message a connected application sends back to the hot-patch dev server.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMsg {
    /// The running process's runtime base address (ASLR slide reference).
    ///
    /// Stub emission needs this so undefined symbols in the patch jump to the
    /// correct runtime addresses. Native sends its pid; wasm omits it.
    AslrReference {
        build_id: u64,
        pid: Option<u32>,
        aslr_reference: u64,
    },
    /// Structured log lines forwarded from the client.
    Log {
        level: String,
        messages: Vec<String>,
    },
}

/// The payload for [`DevserverMsg::HotReload`].
///
/// Field names mirror Dioxus's `HotReloadMsg` so the payload is accepted by
/// Leptos's devtools client. MontRS delivers `view!` patches over its own
/// live-reload socket, so `templates` and `assets` stay empty here.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct HotReloadMsg {
    /// Changed `view!` templates (Dioxus/Leptos wire shape; empty for MontRS).
    #[serde(default)]
    pub templates: Vec<serde_json::Value>,
    /// Changed static assets (Dioxus/Leptos wire shape).
    #[serde(default)]
    pub assets: Vec<PathBuf>,
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

impl RustcInvocation {
    /// The `--crate-name` value, if present.
    pub fn crate_name(&self) -> Option<&str> {
        let mut iter = self.args.iter();
        while let Some(arg) = iter.next() {
            if arg == "--crate-name" {
                return iter.next().map(String::as_str);
            }
            if let Some(rest) = arg.strip_prefix("--crate-name=") {
                return Some(rest);
            }
        }
        None
    }

    /// The argv to replay this invocation directly with `rustc`.
    ///
    /// Skips `argv[0]` (the rustc path) and strips any `-C linker=` override, so
    /// replayed crates produce real outputs instead of re-entering the linker
    /// shim during a patch build.
    pub fn replay_args(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut i = 1;
        while i < self.args.len() {
            let arg = &self.args[i];
            if arg == "-C"
                && self
                    .args
                    .get(i + 1)
                    .is_some_and(|next| next.starts_with("linker="))
            {
                i += 2;
                continue;
            }
            if arg.starts_with("-Clinker=") || arg.starts_with("--linker=") {
                i += 1;
                continue;
            }
            out.push(arg.clone());
            i += 1;
        }
        out
    }
}

/// The crates whose captured `rustc` invocation mentions any changed file.
///
/// A captured invocation lists its crate's source files; intersecting those
/// with the files that just changed tells us which crates must be replayed for
/// a patch.
pub fn changed_crates(
    invocations: &[RustcInvocation],
    changed_files: &[PathBuf],
) -> std::collections::HashSet<String> {
    let changed: std::collections::HashSet<String> = changed_files
        .iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();

    let mut crates = std::collections::HashSet::new();
    for invocation in invocations {
        let Some(name) = invocation.crate_name() else {
            continue;
        };
        let mentions = invocation
            .args
            .iter()
            .any(|arg| changed.contains(&arg.replace('\\', "/")));
        if mentions {
            crates.insert(name.to_string());
        }
    }
    crates
}

/// The link step for the tip binary, captured from the linker wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinkInvocation {
    /// The linker args (the tail passed to the real linker).
    pub args: Vec<String>,
    /// The working directory.
    pub cwd: PathBuf,
    /// Environment the linker ran with (needed so a later patch link finds the
    /// same MSVC/SDK libraries).
    #[serde(default)]
    pub envs: Vec<(String, String)>,
}

/// The directory the wrapper writes capture output to.
pub fn capture_dir() -> PathBuf {
    std::env::var_os("MONTRS_HOTPATCH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/montrs-hotpatch"))
}

static SEQ: AtomicU64 = AtomicU64::new(0);

/// Runtime address of the app's hot-patch cutover function (`call_it`), set by
/// the `serve!` macro when `MONTRS_HOTPATCH_PROBE` is enabled. The client uses
/// it to check whether an incoming table actually contains the cutover entry.
static CUTOVER_KEY: AtomicU64 = AtomicU64::new(0);

/// Record the cutover's runtime key (see [`cutover_key`]).
pub fn set_cutover_key(key: u64) {
    CUTOVER_KEY.store(key, Ordering::Relaxed);
}

/// The cutover's runtime key, or `0` if never recorded.
pub fn cutover_key() -> u64 {
    CUTOVER_KEY.load(Ordering::Relaxed)
}

fn unique_name(kind: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    format!("{kind}-{}-{nanos}-{seq}.json", std::process::id())
}

fn write_json<T: Serialize>(
    dir: &Path,
    name: &str,
    value: &T,
) -> std::io::Result<()> {
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

/// Read every captured link invocation under `<base>/link/`.
pub fn read_link_invocations_in(base: &Path) -> Vec<LinkInvocation> {
    read_all(&base.join("link"))
}

/// Read every captured link invocation from the ambient capture dir.
pub fn read_link_invocations() -> Vec<LinkInvocation> {
    read_link_invocations_in(&capture_dir())
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
    // The linker needs these to locate MSVC/SDK libraries.
    "LIB",
    "LIBPATH",
    "INCLUDE",
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

/// Choose the linker dialect from a target triple.
pub fn flavor_from_triple(triple: &str) -> LinkerFlavor {
    let t = triple.to_ascii_lowercase();
    if t.contains("msvc") || t.contains("windows") {
        LinkerFlavor::Msvc
    } else if t.contains("gnu") || t.contains("linux") || t.contains("musl") {
        LinkerFlavor::Gnu
    } else {
        LinkerFlavor::Other
    }
}

/// Split a linker response file into arguments (MSVC/`CommandLineToArgvW`
/// style: whitespace-separated, double quotes group).
pub fn tokenize_response(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in content.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    out.push(std::mem::take(&mut current));
                }
            }
            c => current.push(c),
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// Read a response file, tolerating UTF-16 (which MSVC's linker writes) as
/// well as UTF-8.
pub fn read_response_file(path: &str) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        Some(String::from_utf16_lossy(&units))
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        Some(String::from_utf16_lossy(&units))
    } else {
        Some(String::from_utf8_lossy(&bytes).into_owned())
    }
}

/// Expand any `@response-file` arguments in place with the file's tokens.
///
/// On Windows rustc passes long link commands via a response file (often
/// UTF-16), so the output path and rlibs are only visible after expansion.
pub fn expand_response_args(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for arg in args {
        if let Some(path) = arg.strip_prefix('@')
            && let Some(content) = read_response_file(path)
        {
            out.extend(tokenize_response(&content));
            continue;
        }
        out.push(arg.clone());
    }
    out
}

/// Write a linker command file containing `args` (each quoted) and return its
/// path, for passing to the linker as `@file` on Windows.
pub fn write_command_file(path: &Path, args: &[String]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let body = args
        .iter()
        .map(|a| format!("\"{a}\""))
        .collect::<Vec<_>>()
        .join(" ");
    std::fs::write(path, body)?;
    Ok(())
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

/// Force an MSVC PDB to be emitted next to the linked output.
///
/// The fat link must carry full symbols because the patch builder reads the
/// running binary's address map from this `.pdb`, but the workspace's
/// line-tables-only dev debuginfo would otherwise produce none.
fn force_msvc_pdb(args: &mut Vec<String>) {
    args.retain(|a| !a.starts_with("/DEBUG"));
    args.push("/DEBUG:FULL".to_string());
    if !args.iter().any(|a| a.starts_with("/PDB:"))
        && let Some(out) = link_output(args)
    {
        args.push(format!("/PDB:{}", out.with_extension("pdb").display()));
    }
}

/// Produce the hot-patchable ("fat") link arguments from a normal link.
///
/// The fat binary exports `main` so a patch can reference the image base, and
/// (on MSVC) disables high-entropy VA so symbol addresses are stable run to
/// run. An optional archive of workspace objects can be force-linked with
/// whole-archive — needed when monomorphized code would otherwise be absent —
/// but plain `link.exe` rejects the GNU-style archive, so it is opt-in.
pub fn fat_link_args(
    original: &[String],
    fat_archive: Option<&Path>,
    flavor: LinkerFlavor,
) -> Vec<String> {
    let mut args = original.to_vec();
    match flavor {
        LinkerFlavor::Msvc => {
            if let Some(archive) = fat_archive {
                args.push(format!("/WHOLEARCHIVE:{}", archive.display()));
            }
            args.push("/EXPORT:main".to_string());
            args.push("/HIGHENTROPYVA:NO".to_string());
            // Incremental linking exports `main` as an ILT thunk whose address
            // does not match the PDB's `main` RVA, which mis-rebases every
            // jump-table entry; the patcher needs direct symbol addresses.
            args.push("/INCREMENTAL:NO".to_string());
            // Keep every function the linked objects define — the running
            // binary is the symbol source for patches, so pruning (e.g. std
            // allocator shims) must not remove them.
            args.push("/OPT:NOREF".to_string());
        }
        LinkerFlavor::Gnu => {
            if let Some(archive) = fat_archive {
                args.push("-Wl,--whole-archive".to_string());
                args.push(archive.display().to_string());
                args.push("-Wl,--no-whole-archive".to_string());
            }
            args.push("-Wl,--export-dynamic-symbol,main".to_string());
        }
        LinkerFlavor::Other => {}
    }
    args
}

/// Copy the tip link's object files into `<base>/tip-objects/`.
///
/// rustc deletes its temporary `.rcgu.o`/`symbols.o` files once the link
/// finishes, so the linker shim saves them here at link time; a later patch
/// links those fresh objects (plus undefined-symbol stubs) into the patch.
/// Returns the saved paths.
pub fn save_tip_objects(
    base: &Path,
    link_args: &[String],
) -> anyhow::Result<Vec<PathBuf>> {
    let dir = base.join("tip-objects");
    std::fs::create_dir_all(&dir)?;
    let mut saved = Vec::new();
    for arg in link_args {
        // Only codegen objects; rustc's `symbols.o` runtime glue is tied to the
        // fat binary and must not be relinked into a patch.
        if !arg.ends_with(".rcgu.o") {
            continue;
        }
        let src = PathBuf::from(arg);
        if !src.is_file() {
            continue;
        }
        let Some(name) = src.file_name() else {
            continue;
        };
        let dst = dir.join(name);
        std::fs::copy(&src, &dst)?;
        saved.push(dst);
    }
    saved.sort();
    saved.dedup();
    Ok(saved)
}

/// The tip objects saved by [`save_tip_objects`].
pub fn read_tip_objects(base: &Path) -> Vec<PathBuf> {
    let dir = base.join("tip-objects");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    out.sort();
    out
}

/// Extract the `.rcgu.o` objects from a set of `.rlib`s into `dir`.
///
/// Passing these objects directly to the linker force-includes them (the
/// whole-archive equivalent) without an archive the MSVC linker would reject.
/// Member names are prefixed with the rlib stem to avoid collisions.
pub fn extract_rlib_objects(
    rlibs: &[PathBuf],
    dir: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(dir)?;
    let mut out = Vec::new();
    for rlib in rlibs {
        let stem = rlib
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "rlib".to_string());
        let bytes = std::fs::read(rlib)?;
        let mut archive = ar::Archive::new(std::io::Cursor::new(bytes));
        while let Some(entry) = archive.next_entry() {
            let Ok(mut entry) = entry else { continue };
            let name = String::from_utf8_lossy(entry.header().identifier())
                .to_string();
            if !name.ends_with(".rcgu.o") {
                continue;
            }
            let out_path = dir.join(format!("{stem}-{name}"));
            let mut data = Vec::new();
            std::io::copy(&mut entry, &mut data)?;
            std::fs::write(&out_path, &data)?;
            out.push(out_path);
        }
    }
    out.sort();
    Ok(out)
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
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(out)?;
    let mut writer = ar::Builder::new(file);
    let mut written = 0usize;

    for rlib in rlibs {
        let bytes = std::fs::read(rlib)?;
        let mut archive = ar::Archive::new(std::io::Cursor::new(bytes));
        while let Some(entry) = archive.next_entry() {
            let Ok(entry) = entry else { continue };
            let name = String::from_utf8_lossy(entry.header().identifier())
                .to_string();
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

/// Assemble the linker arguments for the patch shared library.
///
/// The patch is a DLL/shared object built from just the changed crates' fresh
/// objects plus the undefined-symbol stubs. It deliberately does not link the
/// dependency rlibs — those symbols resolve out of the running binary (via the
/// stubs) when the patch is loaded.
pub fn patch_link_args(
    flavor: LinkerFlavor,
    objects: &[PathBuf],
    original: &[String],
    out: &Path,
) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    match flavor {
        LinkerFlavor::Msvc => {
            args.extend(
                [
                    "shlwapi.lib",
                    "kernel32.lib",
                    "advapi32.lib",
                    "ntdll.lib",
                    "userenv.lib",
                    "ws2_32.lib",
                    "dbghelp.lib",
                    "/defaultlib:msvcrt",
                    "/DLL",
                    "/DEBUG",
                    "/PDBALTPATH:%_PDB%",
                    "/EXPORT:main",
                    "/HIGHENTROPYVA:NO",
                    // Direct symbol addresses: an ILT thunk for `main` would
                    // not match the PDB RVA `apply_patch` rebases against.
                    "/INCREMENTAL:NO",
                ]
                .iter()
                .map(|s| s.to_string()),
            );
            // Preserve library search paths so the Windows SDK / MSVC libs
            // resolve when linking the patch directly.
            let mut i = 0;
            while i < original.len() {
                let arg = &original[i];
                if arg.starts_with("/LIBPATH:") {
                    args.push(arg.clone());
                }
                if arg == "-L" && i + 1 < original.len() {
                    args.push(arg.clone());
                    args.push(original[i + 1].clone());
                    i += 2;
                    continue;
                }
                i += 1;
            }
            args.extend(objects.iter().map(|p| p.display().to_string()));
            args.push(format!("/OUT:{}", out.display()));
        }
        LinkerFlavor::Gnu => {
            args.extend(
                [
                    "-shared",
                    "-Wl,--eh-frame-hdr",
                    "-Wl,-z,noexecstack",
                    "-Wl,-z,relro,-z,now",
                    "-nodefaultlibs",
                    "-Wl,-Bdynamic",
                ]
                .iter()
                .map(|s| s.to_string()),
            );
            // Preserve library search paths and libs from the fat link.
            let mut i = 0;
            while i < original.len() {
                let arg = &original[i];
                if arg == "-L" && i + 1 < original.len() {
                    args.push(arg.clone());
                    args.push(original[i + 1].clone());
                    i += 2;
                    continue;
                }
                if arg.starts_with("-l")
                    || arg.starts_with("-m")
                    || arg.starts_with("-B")
                {
                    args.push(arg.clone());
                }
                i += 1;
            }
            args.extend(objects.iter().map(|p| p.display().to_string()));
            args.push("-o".to_string());
            args.push(out.display().to_string());
        }
        LinkerFlavor::Other => {}
    }
    args
}

/// The `.rlib` paths in `link_args` that live under the workspace target dir
/// (i.e. workspace crates), excluding sysroot/toolchain rlibs.
pub fn workspace_rlibs(
    link_args: &[String],
    workspace_target_dir: &Path,
) -> Vec<PathBuf> {
    link_rlibs(link_args)
        .into_iter()
        .filter(|p| p.starts_with(workspace_target_dir))
        .collect()
}

/// Replace (or add) the output path in link args for the given flavor.
pub fn set_link_output(flavor: LinkerFlavor, args: &mut [String], out: &Path) {
    match flavor {
        LinkerFlavor::Msvc | LinkerFlavor::Other => {
            if let Some(pos) = args.iter().position(|a| a.starts_with("/OUT:"))
            {
                args[pos] = format!("/OUT:{}", out.display());
            }
        }
        LinkerFlavor::Gnu => {
            if let Some(pos) = args.iter().position(|a| a == "-o")
                && pos + 1 < args.len()
            {
                args[pos + 1] = out.display().to_string();
            }
        }
    }
}

/// Remove the given paths from a link command.
pub fn without_paths(args: &[String], paths: &[PathBuf]) -> Vec<String> {
    let remove: std::collections::HashSet<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    args.iter()
        .filter(|a| !remove.contains(a.as_str()))
        .cloned()
        .collect()
}

/// Re-link a fat binary in place with the hot-patch metadata.
///
/// Builds a fat archive from the workspace rlibs in `link_args` (removing those
/// rlibs from the command, since the archive replaces them), links to a
/// temporary path, and only then swaps it over `output` — so a failed relink
/// never corrupts the working binary. Returns the number of objects archived.
pub fn relink_fat(
    real_linker: &Path,
    flavor: LinkerFlavor,
    original_link_args: &[String],
    _workspace_target_dir: &Path,
    _fat_archive: &Path,
    output: &Path,
) -> anyhow::Result<usize> {
    let written = 0;
    let temp = output.with_extension("fatlinking");
    let mut args = fat_link_args(original_link_args, None, flavor);
    set_link_output(flavor, &mut args, &temp);

    let result = std::process::Command::new(real_linker)
        .args(&args)
        .output()?;
    if !result.status.success() {
        let _ = std::fs::remove_file(&temp);
        // MSVC's link.exe tends to write diagnostics to stdout, not stderr.
        let mut msg =
            String::from_utf8_lossy(&result.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&result.stdout);
        if !stdout.trim().is_empty() {
            if !msg.is_empty() {
                msg.push('\n');
            }
            msg.push_str(stdout.trim());
        }
        anyhow::bail!("fat relink failed ({}): {msg}", result.status);
    }
    std::fs::rename(&temp, output)?;
    Ok(written)
}

/// Fat-link in place: archive the workspace rlibs, drop them from the command,
/// add the hot-patch flags, and run the real linker to the output named in
/// `link_args`.
///
/// This is meant to run *inside* the linker shim during the tip link, where
/// rustc's temporary object files still exist (a post-build relink cannot work
/// because rustc removes them).
pub fn fat_link_in_place(
    real_linker: &Path,
    flavor: LinkerFlavor,
    link_args: &[String],
    workspace_target_dir: &Path,
    fat_archive: &Path,
) -> anyhow::Result<usize> {
    // Force-include the workspace objects so symbols that `link.exe`'s
    // `/OPT:REF` would otherwise prune (e.g. std alloc shims) stay in the fat
    // binary's symbol table.
    let rlibs = workspace_rlibs(link_args, workspace_target_dir);
    let obj_dir = fat_archive.with_extension("objects");
    let objects = extract_rlib_objects(&rlibs, &obj_dir)?;
    let mut base = without_paths(link_args, &rlibs);
    base.extend(objects.iter().map(|p| p.display().to_string()));

    let mut args = fat_link_args(&base, None, flavor);
    if flavor == LinkerFlavor::Msvc {
        // The patch builder indexes the running binary's symbols from its
        // `.pdb`, but `[profile.dev] debug = 1` (line-tables-only) makes MSVC
        // emit none. Force a full one for the fat link.
        force_msvc_pdb(&mut args);
    }

    // Windows link commands routinely exceed the command-line limit, so the
    // extended argument set goes through a command file.
    let result = if cfg!(windows) {
        let cmd_file = fat_archive.with_file_name("fat-link-args.txt");
        write_command_file(&cmd_file, &args)?;
        std::process::Command::new(real_linker)
            .arg(format!("@{}", cmd_file.display()))
            .output()?
    } else {
        std::process::Command::new(real_linker)
            .args(&args)
            .output()?
    };
    if !result.status.success() {
        let mut msg =
            String::from_utf8_lossy(&result.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&result.stdout);
        if !stdout.trim().is_empty() {
            if !msg.is_empty() {
                msg.push('\n');
            }
            msg.push_str(stdout.trim());
        }
        anyhow::bail!("fat link failed ({}): {msg}", result.status);
    }
    Ok(objects.len())
}

/// Everything needed to build one patch.
pub struct PatchRequest<'a> {
    /// The capture directory (`MONTRS_HOTPATCH_DIR`).
    pub capture_base: &'a Path,
    /// The running ("fat") binary.
    pub exe: &'a Path,
    /// The workspace target directory, used to tell workspace crates from
    /// sysroot ones when choosing patch-link fallback libraries.
    pub workspace_target_dir: &'a Path,
    /// The linker that will link the patch DLL.
    pub real_linker: &'a Path,
    pub flavor: LinkerFlavor,
    /// The running process's runtime base address.
    pub aslr_reference: u64,
    /// The client's build id (wasm reports `0`).
    pub build_id: u64,
    /// The target process id, if native.
    pub pid: Option<u32>,
}

/// Build a patch from the saved tip objects and the running binary's symbols.
///
/// Emits undefined-symbol stubs addressed at the running process, links them
/// with the tip objects into a patch shared library, and returns the jump table
/// the client applies.
pub fn build_patch(request: &PatchRequest) -> anyhow::Result<JumpTable> {
    let base = request.capture_base;

    let fat = SymbolIndex::from_exe(request.exe)?;
    let aslr_ref_address = fat
        .get("main")
        .ok_or_else(|| anyhow::anyhow!("fat binary has no `main` symbol"))?;
    let aslr_offset = request.aslr_reference.wrapping_sub(aslr_ref_address);

    let objects = read_tip_objects(base);
    if objects.is_empty() {
        anyhow::bail!(
            "no tip objects captured; run a build with MONTRS_HOTPATCH=1 first"
        );
    }

    let undefined = undefined_symbols(&objects)?;
    let stub_bytes =
        emit_undefined_symbol_stubs(&undefined, &fat, aslr_offset)?;
    let stub_path = base.join("patch-stubs.obj");
    std::fs::write(&stub_path, stub_bytes)?;

    let patch_lib = base.join(if cfg!(windows) {
        "libpatch.dll"
    } else {
        "libpatch.so"
    });
    let latest = read_latest_link_in(base);
    let original = latest.as_ref().map(|l| l.args.clone()).unwrap_or_default();
    let envs = latest.as_ref().map(|l| l.envs.clone()).unwrap_or_default();

    let mut link_objects = objects;
    link_objects.push(stub_path);

    let args =
        patch_link_args(request.flavor, &link_objects, &original, &patch_lib);
    run_linker(request.real_linker, &args, &envs)?;

    let patch_symbols = SymbolIndex::from_exe(&patch_lib)?;
    build_jump_table(patch_lib, &fat, &patch_symbols)
}

/// Run a linker to completion, surfacing stdout+stderr on failure.
fn run_linker(
    linker: &Path,
    args: &[String],
    envs: &[(String, String)],
) -> anyhow::Result<()> {
    let result = if cfg!(windows) {
        let cmd = std::env::temp_dir()
            .join(format!("montrs-patch-{}.txt", std::process::id()));
        write_command_file(&cmd, args)?;
        let output = std::process::Command::new(linker)
            .arg(format!("@{}", cmd.display()))
            .envs(envs.iter().cloned())
            .output()?;
        let _ = std::fs::remove_file(&cmd);
        output
    } else {
        std::process::Command::new(linker)
            .args(args)
            .envs(envs.iter().cloned())
            .output()?
    };

    if !result.status.success() {
        let mut msg =
            String::from_utf8_lossy(&result.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&result.stdout);
        if !stdout.trim().is_empty() {
            if !msg.is_empty() {
                msg.push('\n');
            }
            msg.push_str(stdout.trim());
        }
        anyhow::bail!("patch link failed ({}): {msg}", result.status);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Patch object symbols
// ---------------------------------------------------------------------------

/// Collect the `(defined, undefined)` symbol names from an object file.
pub fn object_symbols(
    path: &Path,
) -> anyhow::Result<(
    std::collections::HashSet<String>,
    std::collections::HashSet<String>,
)> {
    use object::{Object, ObjectSymbol};

    let data = std::fs::read(path)?;
    let file = object::File::parse(&*data)?;
    let mut defined = std::collections::HashSet::new();
    let mut undefined = std::collections::HashSet::new();
    for symbol in file.symbols() {
        let Ok(name) = symbol.name() else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        if symbol.is_undefined() {
            undefined.insert(name.to_string());
        } else {
            defined.insert(name.to_string());
        }
    }
    Ok((defined, undefined))
}

/// The symbols a set of patch objects reference but do not define.
///
/// These are exactly the symbols undefined-symbol stubs must satisfy by
/// jumping into the running binary's known addresses.
pub fn undefined_symbols(objects: &[PathBuf]) -> anyhow::Result<Vec<String>> {
    let mut defined = std::collections::HashSet::new();
    let mut undefined = std::collections::HashSet::new();
    for path in objects {
        let (d, u) = object_symbols(path)?;
        defined.extend(d);
        undefined.extend(u);
    }
    let mut out: Vec<String> =
        undefined.difference(&defined).cloned().collect();
    out.sort();
    Ok(out)
}

// ---------------------------------------------------------------------------
// Wasm symbols
// ---------------------------------------------------------------------------

/// Exported function symbols of a wasm module: `name -> function id`.
///
/// This is the foundation of the wasm jump table: a patch maps the running
/// (fat) module's function ids to the freshly linked patch's ids by name. The
/// full wasm patcher additionally prepares the base module (promoting every
/// function into the indirect-function table) and resolves `GOT.func`/
/// `GOT.mem`/`__wbindgen_placeholder__` imports.
pub fn wasm_function_symbols(
    bytes: &[u8],
) -> anyhow::Result<std::collections::HashMap<String, u32>> {
    let module = walrus::Module::from_buffer(bytes)?;
    let mut map = std::collections::HashMap::new();
    for export in module.exports.iter() {
        if let walrus::ExportItem::Function(id) = export.item {
            map.insert(export.name.clone(), id.index() as u32);
        }
    }
    Ok(map)
}

/// Build a wasm [`JumpTable`] mapping the fat module's function ids to the
/// patch module's, keyed by exported name.
pub fn build_wasm_jump_table(
    lib: PathBuf,
    fat_wasm: &[u8],
    patch_wasm: &[u8],
) -> anyhow::Result<JumpTable> {
    let fat = wasm_function_symbols(fat_wasm)?;
    let patch = wasm_function_symbols(patch_wasm)?;

    let mut map = subsecond_types::AddressMap::default();
    for (name, new_id) in &patch {
        if let Some(old_id) = fat.get(name) {
            map.insert(u64::from(*old_id), u64::from(*new_id));
        }
    }
    let ifunc_count = patch.len() as u64;

    Ok(JumpTable {
        lib,
        map,
        aslr_reference: 0,
        new_base_address: 0,
        ifunc_count,
    })
}

// ---------------------------------------------------------------------------
// Fat-binary symbol index
// ---------------------------------------------------------------------------

/// A symbol's kind, enough to choose a stub shape (jump vs. data word).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// Executable code.
    Text,
    /// Data.
    Data,
    /// Unknown / not important.
    Unknown,
}

/// One indexed symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedSymbol {
    pub address: u64,
    pub kind: SymbolKind,
    pub is_undefined: bool,
}

/// A `name → symbol` map of a binary's symbols.
///
/// A hot patch is linked against the running process's known symbol addresses,
/// so the patcher first reads them: from the sibling `.pdb` on Windows, or the
/// executable's object symbol table elsewhere.
#[derive(Debug, Default)]
pub struct SymbolIndex {
    symbols: std::collections::HashMap<String, IndexedSymbol>,
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
                            IndexedSymbol {
                                address: rva.0 as u64,
                                kind: if data.function {
                                    SymbolKind::Text
                                } else {
                                    SymbolKind::Data
                                },
                                is_undefined: false,
                            },
                        );
                    }
                }
                Ok(pdb::SymbolData::Data(data)) => {
                    if let Some(rva) = data.offset.to_rva(&address_map) {
                        symbols.insert(
                            data.name.to_string().to_string(),
                            IndexedSymbol {
                                address: rva.0 as u64,
                                kind: SymbolKind::Data,
                                is_undefined: false,
                            },
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
                symbols.insert(
                    name.to_string(),
                    IndexedSymbol {
                        address: symbol.address(),
                        kind: match symbol.kind() {
                            object::SymbolKind::Text => SymbolKind::Text,
                            object::SymbolKind::Data => SymbolKind::Data,
                            _ => SymbolKind::Unknown,
                        },
                        is_undefined: symbol.is_undefined(),
                    },
                );
            }
        }
        Ok(Self { symbols })
    }

    /// Build an index directly from `name → address` pairs (kind `Text`).
    pub fn from_pairs<I: IntoIterator<Item = (String, u64)>>(pairs: I) -> Self {
        Self {
            symbols: pairs
                .into_iter()
                .map(|(name, address)| {
                    (
                        name,
                        IndexedSymbol {
                            address,
                            kind: SymbolKind::Text,
                            is_undefined: false,
                        },
                    )
                })
                .collect(),
        }
    }

    /// Build an index from fully-described symbols.
    pub fn from_indexed<I: IntoIterator<Item = (String, IndexedSymbol)>>(
        pairs: I,
    ) -> Self {
        Self {
            symbols: pairs.into_iter().collect(),
        }
    }

    /// Iterate over `(name, symbol)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &IndexedSymbol)> {
        self.symbols.iter()
    }

    /// Look up a symbol's address.
    pub fn get(&self, name: &str) -> Option<u64> {
        self.symbols.get(name).map(|s| s.address)
    }

    /// Look up a symbol's full record.
    pub fn symbol(&self, name: &str) -> Option<&IndexedSymbol> {
        self.symbols.get(name)
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
    for (name, new_sym) in patch.iter() {
        if let Some(old_addr) = fat.get(name) {
            map.insert(old_addr, new_sym.address);
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

/// Emit an object file that satisfies patch `undefined` symbols by jumping to
/// (text) or pointing at (data) their addresses in the running binary.
///
/// `aslr_offset` is `runtime_base - fat_main_rva`, which turns the fat binary's
/// relative symbol addresses into the runtime absolute addresses the patch must
/// call. x86_64 text symbols get a `mov rax, imm64; jmp rax` trampoline; data
/// symbols (and Windows `__imp_` pointers) get an 8-byte absolute word.
pub fn emit_undefined_symbol_stubs(
    undefined: &[String],
    fat: &SymbolIndex,
    aslr_offset: u64,
) -> anyhow::Result<Vec<u8>> {
    use object::{
        Architecture, BinaryFormat, Endianness, SymbolFlags,
        SymbolKind as ObjKind, SymbolScope,
        write::{Object, StandardSection, Symbol, SymbolSection},
    };

    let mut obj = Object::new(
        BinaryFormat::Coff,
        Architecture::X86_64,
        Endianness::Little,
    );
    let text = obj.section_id(StandardSection::Text);
    let data = obj.section_id(StandardSection::Data);
    let mut text_bytes: Vec<u8> = Vec::new();
    let mut data_bytes: Vec<u8> = Vec::new();

    for name in undefined {
        // `__imp_x` is a pointer to `x`; resolve through it.
        let lookup = name.strip_prefix("__imp_").unwrap_or(name);
        let Some(sym) = fat.symbol(lookup) else {
            continue;
        };
        if sym.is_undefined {
            continue;
        }
        let absolute = sym.address.wrapping_add(aslr_offset);

        if name.starts_with("__imp_") || sym.kind == SymbolKind::Data {
            let offset = data_bytes.len() as u64;
            data_bytes.extend_from_slice(&absolute.to_le_bytes());
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value: offset,
                size: 8,
                kind: ObjKind::Data,
                scope: SymbolScope::Linkage,
                weak: false,
                flags: SymbolFlags::None,
                section: SymbolSection::Section(data),
            });
        } else if sym.kind == SymbolKind::Text {
            let offset = text_bytes.len() as u64;
            // mov rax, imm64 ; jmp rax
            text_bytes.extend_from_slice(&[0x48, 0xB8]);
            text_bytes.extend_from_slice(&absolute.to_le_bytes());
            text_bytes.extend_from_slice(&[0xFF, 0xE0]);
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value: offset,
                size: 12,
                kind: ObjKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                flags: SymbolFlags::None,
                section: SymbolSection::Section(text),
            });
        }
    }

    if !text_bytes.is_empty() {
        obj.append_section_data(text, &text_bytes, 1);
    }
    if !data_bytes.is_empty() {
        obj.append_section_data(data, &data_bytes, 8);
    }
    Ok(obj.write()?)
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
            envs: vec![("LIB".into(), "C:\\sdk\\lib".into())],
        };
        capture_link_in(&dir, &inv).unwrap();
        assert_eq!(read_latest_link_in(&dir), Some(inv));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parses_linker_from_print_link_args() {
        let sample = "\"C:\\\\Program Files\\\\Microsoft Visual \
                      Studio\\\\2022\\\\VC\\\\Tools\\\\MSVC\\\\14.44.35207\\\\\
                      bin\\\\HostX64\\\\x64\\\\link.exe\" \"/NOLOGO\" \
                      \"/OUT:probe.exe\"";
        let linker = parse_linker_from_link_args(sample).expect("linker");
        assert_eq!(
            linker,
            PathBuf::from(
                "C:\\Program Files\\Microsoft Visual \
                 Studio\\2022\\VC\\Tools\\MSVC\\14.44.35207\\bin\\HostX64\\\
                 x64\\link.exe"
            )
        );
        assert!(parse_linker_from_link_args("not quoted").is_none());
    }

    #[test]
    fn expands_response_files() {
        let dir = temp_dir("hp-resp");
        std::fs::create_dir_all(&dir).unwrap();
        let rf = dir.join("args.txt");
        std::fs::write(
            &rf,
            "\"/NOLOGO\" \"/OUT:C:\\a b\\x.exe\" \"libfoo.rlib\"",
        )
        .unwrap();
        let args = vec![format!("@{}", rf.display()), "/DEBUG".to_string()];
        let out = expand_response_args(&args);
        assert_eq!(
            out,
            vec![
                "/NOLOGO".to_string(),
                "/OUT:C:\\a b\\x.exe".to_string(),
                "libfoo.rlib".to_string(),
                "/DEBUG".to_string(),
            ]
        );
        assert_eq!(link_output(&out), Some(PathBuf::from("C:\\a b\\x.exe")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn without_paths_drops_archive_members() {
        let args = vec![
            "/NOLOGO".to_string(),
            "libapp.rlib".to_string(),
            "libui.rlib".to_string(),
            "C:\\rust\\libstd.rlib".to_string(),
        ];
        let out = without_paths(
            &args,
            &[PathBuf::from("libapp.rlib"), PathBuf::from("libui.rlib")],
        );
        assert_eq!(
            out,
            vec!["/NOLOGO".to_string(), "C:\\rust\\libstd.rlib".to_string()]
        );
    }

    #[test]
    fn flavor_from_triple_maps_known_targets() {
        assert_eq!(
            flavor_from_triple("x86_64-pc-windows-msvc"),
            LinkerFlavor::Msvc
        );
        assert_eq!(
            flavor_from_triple("x86_64-unknown-linux-gnu"),
            LinkerFlavor::Gnu
        );
        assert_eq!(
            flavor_from_triple("aarch64-apple-darwin"),
            LinkerFlavor::Other
        );
    }

    #[test]
    fn workspace_rlibs_filter_by_prefix() {
        let ws = PathBuf::from("C:\\proj\\target");
        let args = vec![
            "C:\\proj\\target\\debug\\deps\\libapp-1.rlib".to_string(),
            "C:\\rust\\lib\\rustlib\\...\\libstd.rlib".to_string(),
            "C:\\proj\\target\\debug\\deps\\libui-2.rlib".to_string(),
            "-o".to_string(),
            "app.exe".to_string(),
        ];
        let rlibs = workspace_rlibs(&args, &ws);
        assert_eq!(rlibs.len(), 2);
        assert!(rlibs.iter().all(|p| p.starts_with(&ws)));
    }

    #[test]
    fn set_link_output_replaces_msvc_and_gnu_outputs() {
        let mut msvc = vec!["/NOLOGO".to_string(), "/OUT:a.exe".to_string()];
        set_link_output(LinkerFlavor::Msvc, &mut msvc, Path::new("b.exe"));
        assert_eq!(link_output(&msvc), Some(PathBuf::from("b.exe")));

        let mut gnu = vec!["-o".to_string(), "a".to_string()];
        set_link_output(LinkerFlavor::Gnu, &mut gnu, Path::new("b"));
        assert_eq!(link_output(&gnu), Some(PathBuf::from("b")));
    }

    #[test]
    fn wasm_jump_table_maps_functions_by_name() {
        fn module_with(exports: &[&str]) -> Vec<u8> {
            let mut module = walrus::Module::default();
            for name in exports {
                let builder =
                    walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
                let fid = builder.finish(vec![], &mut module.funcs);
                module.exports.add(name, fid);
            }
            module.emit_wasm()
        }

        let fat = module_with(&["a", "b", "c"]);
        let patch = module_with(&["a", "c", "d"]);
        assert_eq!(wasm_function_symbols(&fat).unwrap().len(), 3);

        let table =
            build_wasm_jump_table(PathBuf::from("patch.wasm"), &fat, &patch)
                .unwrap();
        assert_eq!(table.map.len(), 2, "only names in both modules map");
        assert_eq!(table.ifunc_count, 3);
    }

    #[test]
    fn expands_utf16_response_files() {
        let dir = temp_dir("hp-resp16");
        std::fs::create_dir_all(&dir).unwrap();
        let rf = dir.join("args.txt");
        let text = "\"/OUT:x.exe\" \"a.obj\"";
        let mut bytes = vec![0xFFu8, 0xFE];
        for unit in text.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        std::fs::write(&rf, bytes).unwrap();
        let args = vec![format!("@{}", rf.display())];
        let out = expand_response_args(&args);
        assert_eq!(link_output(&out), Some(PathBuf::from("x.exe")));
        assert!(out.contains(&"a.obj".to_string()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn extracts_rcgu_objects_from_rlibs() {
        let dir = temp_dir("hp-extract");
        std::fs::create_dir_all(&dir).unwrap();
        let rlib = dir.join("libfoo.rlib");
        {
            let file = std::fs::File::create(&rlib).unwrap();
            let mut builder = ar::Builder::new(file);
            let obj = b"OBJ";
            builder
                .append(
                    &ar::Header::new(b"foo.rcgu.o".to_vec(), obj.len() as u64),
                    &obj[..],
                )
                .unwrap();
            let meta = b"M";
            builder
                .append(
                    &ar::Header::new(b"foo.rmeta".to_vec(), meta.len() as u64),
                    &meta[..],
                )
                .unwrap();
        }
        let objects =
            extract_rlib_objects(&[rlib], &dir.join("objects")).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(
            objects[0]
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains("foo.rcgu.o")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn saves_and_reads_tip_objects() {
        let base = temp_dir("hp-tipobj");
        let src = base.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("a.rcgu.o"), b"a").unwrap();
        std::fs::write(src.join("symbols.o"), b"s").unwrap();
        std::fs::write(src.join("skip.rlib"), b"r").unwrap();
        let args = vec![
            src.join("a.rcgu.o").to_string_lossy().into_owned(),
            src.join("symbols.o").to_string_lossy().into_owned(),
            src.join("skip.rlib").to_string_lossy().into_owned(),
        ];
        let saved = save_tip_objects(&base, &args).unwrap();
        assert_eq!(saved.len(), 1, "only .rcgu.o is saved");
        let read = read_tip_objects(&base);
        assert_eq!(read.len(), 1);
        assert!(
            read.iter()
                .all(|p| p.parent().unwrap().ends_with("tip-objects"))
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn replay_args_strip_linker_overrides() {
        let invocation = RustcInvocation {
            args: vec![
                "rustc".into(),
                "-C".into(),
                "linker=C:\\w\\shim.exe".into(),
                "-Clinker=other".into(),
                "--crate-name".into(),
                "app".into(),
                "--emit=metadata".into(),
            ],
            cwd: PathBuf::from("/w"),
            envs: vec![],
        };
        assert_eq!(
            invocation.replay_args(),
            vec![
                "--crate-name".to_string(),
                "app".to_string(),
                "--emit=metadata".to_string()
            ]
        );
    }

    #[test]
    fn changed_files_map_to_their_crates() {
        let invocations = vec![
            RustcInvocation {
                args: vec![
                    "rustc".into(),
                    "--crate-name".into(),
                    "app".into(),
                    "app/src/main.rs".into(),
                ],
                cwd: PathBuf::from("/w"),
                envs: vec![],
            },
            RustcInvocation {
                args: vec![
                    "rustc".into(),
                    "--crate-name".into(),
                    "montrs_ui".into(),
                    "packages/ui/src/lib.rs".into(),
                ],
                cwd: PathBuf::from("/w"),
                envs: vec![],
            },
        ];
        assert_eq!(invocations[0].crate_name(), Some("app"));

        let changed = vec![
            PathBuf::from("app/src/main.rs"),
            PathBuf::from("packages\\ui\\src\\lib.rs"),
        ];
        let crates = changed_crates(&invocations, &changed);
        assert!(crates.contains("app"));
        assert!(crates.contains("montrs_ui"));
        assert_eq!(crates.len(), 2);

        let none = changed_crates(&invocations, &[PathBuf::from("other.rs")]);
        assert!(none.is_empty());
    }

    #[test]
    fn patch_link_args_msvc_is_a_dll_with_export_and_output() {
        let objects = vec![PathBuf::from("a.obj"), PathBuf::from("stub.obj")];
        let original = vec!["/LIBPATH:C:\\sdk\\lib".to_string()];
        let args = patch_link_args(
            LinkerFlavor::Msvc,
            &objects,
            &original,
            Path::new("libpatch.dll"),
        );
        assert!(args.contains(&"/DLL".to_string()));
        assert!(args.contains(&"/EXPORT:main".to_string()));
        assert!(args.contains(&"a.obj".to_string()));
        assert!(args.contains(&"stub.obj".to_string()));
        assert!(args.contains(&"/OUT:libpatch.dll".to_string()));
        assert!(args.contains(&"/LIBPATH:C:\\sdk\\lib".to_string()));
    }

    #[test]
    fn patch_link_args_gnu_is_shared_and_preserves_libs() {
        let objects = vec![PathBuf::from("a.o")];
        let original = vec![
            "-L".to_string(),
            "/opt/lib".to_string(),
            "-lfoo".to_string(),
            "kernel32.lib".to_string(),
        ];
        let args = patch_link_args(
            LinkerFlavor::Gnu,
            &objects,
            &original,
            Path::new("libpatch.so"),
        );
        assert!(args.contains(&"-shared".to_string()));
        assert!(args.contains(&"-L".to_string()));
        assert!(args.contains(&"/opt/lib".to_string()));
        assert!(args.contains(&"-lfoo".to_string()));
        assert!(args.contains(&"a.o".to_string()));
        let n = args.len();
        assert_eq!(
            &args[n - 2..],
            &["-o".to_string(), "libpatch.so".to_string()]
        );
    }

    #[test]
    fn emits_text_and_data_stubs() {
        use object::{Object, ObjectSection, ObjectSymbol};

        let fat = SymbolIndex::from_indexed([
            (
                "some_fn".to_string(),
                IndexedSymbol {
                    address: 0x1000,
                    kind: SymbolKind::Text,
                    is_undefined: false,
                },
            ),
            (
                "some_data".to_string(),
                IndexedSymbol {
                    address: 0x2000,
                    kind: SymbolKind::Data,
                    is_undefined: false,
                },
            ),
            (
                "stat".to_string(),
                IndexedSymbol {
                    address: 0x3000,
                    kind: SymbolKind::Data,
                    is_undefined: false,
                },
            ),
        ]);
        let undefined = vec![
            "some_fn".to_string(),
            "some_data".to_string(),
            "__imp_stat".to_string(),
        ];

        let bytes =
            emit_undefined_symbol_stubs(&undefined, &fat, 0x4000).unwrap();
        let file = object::File::parse(&*bytes).unwrap();

        let mut names = std::collections::HashSet::new();
        for symbol in file.symbols() {
            if let Ok(name) = symbol.name() {
                names.insert(name.to_string());
            }
        }
        assert!(names.contains("some_fn"));
        assert!(names.contains("some_data"));
        assert!(names.contains("__imp_stat"));

        // Text stub: mov rax, 0x5000 ; jmp rax
        let text = file.section_by_name(".text").unwrap().data().unwrap();
        assert_eq!(&text[0..2], &[0x48, 0xB8]);
        assert_eq!(u64::from_le_bytes(text[2..10].try_into().unwrap()), 0x5000);
        assert_eq!(&text[10..12], &[0xFF, 0xE0]);

        // Data stubs: absolute words for some_data and __imp_stat.
        let data = file.section_by_name(".data").unwrap().data().unwrap();
        assert_eq!(u64::from_le_bytes(data[0..8].try_into().unwrap()), 0x6000);
        assert_eq!(u64::from_le_bytes(data[8..16].try_into().unwrap()), 0x7000);
    }

    #[test]
    fn client_aslr_message_round_trips() {
        let msg = ClientMsg::AslrReference {
            build_id: 0,
            pid: Some(42),
            aslr_reference: 0x1_4000_0000,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"aslr_reference\""), "{json}");
        let back: ClientMsg = serde_json::from_str(&json).unwrap();
        assert_eq!(back, msg);
    }

    #[test]
    fn collects_undefined_symbols_from_objects() {
        use object::{
            Architecture, BinaryFormat, Endianness, SymbolFlags, SymbolKind,
            SymbolScope,
            write::{Object, StandardSection, Symbol, SymbolSection},
        };

        let dir = temp_dir("hp-obj");
        std::fs::create_dir_all(&dir).unwrap();
        let obj_path = dir.join("patch.obj");
        {
            let mut obj = Object::new(
                BinaryFormat::Coff,
                Architecture::X86_64,
                Endianness::Little,
            );
            let text = obj.section_id(StandardSection::Text);
            obj.add_symbol(Symbol {
                name: b"defined_fn".to_vec(),
                value: 0,
                size: 0,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                flags: SymbolFlags::None,
                section: SymbolSection::Section(text),
            });
            obj.add_symbol(Symbol {
                name: b"undefined_fn".to_vec(),
                value: 0,
                size: 0,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                flags: SymbolFlags::None,
                section: SymbolSection::Undefined,
            });
            obj.append_section_data(text, &[0xC3], 1);
            std::fs::write(&obj_path, obj.write().unwrap()).unwrap();
        }

        let undefined = undefined_symbols(&[obj_path]).unwrap();
        assert!(undefined.contains(&"undefined_fn".to_string()));
        assert!(!undefined.contains(&"defined_fn".to_string()));

        let _ = std::fs::remove_dir_all(&dir);
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
        let original = vec!["/NOLOGO".to_string(), "/OUT:app.exe".to_string()];
        let args = fat_link_args(&original, None, LinkerFlavor::Msvc);
        assert!(args.iter().any(|a| a == "/EXPORT:main"));
        assert!(args.iter().any(|a| a == "/HIGHENTROPYVA:NO"));
        // Must not prune: the running binary is the patch's symbol source.
        assert!(args.iter().any(|a| a == "/OPT:NOREF"));
        assert_eq!(link_output(&args), Some(PathBuf::from("app.exe")));

        // Opt-in whole-archive.
        let with_archive = fat_link_args(
            &original,
            Some(Path::new("deps.a")),
            LinkerFlavor::Msvc,
        );
        assert!(with_archive.iter().any(|a| a.starts_with("/WHOLEARCHIVE:")));
    }

    #[test]
    fn fat_link_args_gnu_adds_export_and_optional_archive() {
        let original = vec!["-o".to_string(), "app".to_string()];
        let args = fat_link_args(&original, None, LinkerFlavor::Gnu);
        assert!(args.contains(&"-Wl,--export-dynamic-symbol,main".to_string()));
        let with_archive = fat_link_args(
            &original,
            Some(Path::new("libdeps.a")),
            LinkerFlavor::Gnu,
        );
        assert!(with_archive.contains(&"-Wl,--whole-archive".to_string()));
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
        let mut map = subsecond_types::AddressMap::default();
        map.insert(0x1000, 0x2000);
        let msg = DevserverMsg::HotReload(HotReloadMsg {
            templates: Vec::new(),
            assets: Vec::new(),
            jump_table: Some(JumpTable {
                lib: PathBuf::from("libpatch.dll"),
                map,
                aslr_reference: 0x7ff6_0000_0000,
                new_base_address: 0x1000,
                ifunc_count: 0,
            }),
            ms_elapsed: 12,
            for_build_id: Some(0),
            for_pid: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        // Externally tagged, matching Dioxus/Leptos.
        assert!(json.contains("\"HotReload\""), "{json}");
        // A populated address map must survive the round trip; this is the case
        // Serde's internally-tagged form silently corrupted.
        let back: DevserverMsg = serde_json::from_str(&json).unwrap();
        assert_eq!(back, msg);
    }
}

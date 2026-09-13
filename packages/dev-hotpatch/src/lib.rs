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

#[cfg(test)]
mod tests {
    use super::*;

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
